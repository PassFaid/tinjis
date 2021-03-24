# Tinjis

## Instructions

You'll need to have the following installed: `docker`, `docker-compose`, `make`, `kubectl`, and optionally [`dhall-to-yaml`](https://github.com/dhall-lang/dhall-haskell/releases/tag/1.38.0).

Assuming you have a kubernetes (k8s) cluster (with ingress configured, and the ability to pull public images from dockerhub), and `kubectl` pointed at it, do the following to run the solution:

1. Generate the kubernetes yaml files: `make render-k8s-yaml-docker`
2. Apply it to kubernetes with: `make deploy`
3. Wait for services to become available

For testing the solution, the configured ingress forwards all paths on port `80`, to the antaeus service, so assuming your k8s cluster is local and your ingress is configured to make port `80` available, the following should work:

```
curl -v 127.0.0.1:80/rest/v1/invoices # check initial invoices
curl -v 127.0.0.1:80/rest/v1/invoices/pay -X POST # pay some invoices
curl -v 127.0.0.1:80/rest/v1/invoices # check to see if any have been paid
```

There's also a test client that runs the above (source in the `/testclient` dir), which you can run with:

```
kubectl run testclient --env="MAX_RETRIES=30" --env="API_URL=http://antaeus" --restart=Never --rm -i --image snazzybucket/testclient:latest
```

where the value `API_URL` is a publicly available address of the antaeus service. I'm not sure about your network setup, but to see this client running both in kubernetes and externally via the ingress, you can take a look at the [Github Action job here](https://github.com/alexhumphreys/tinjis/runs/2178964320).

## Details

### `provider`

The payment provider microservice is a Rust server located in the `/provider` dir. You can run both it and antaeus together with `docker-compose up`. It has two endpoints:

- `GET /health`
- `POST /api/pay`

### `testclient`

The test client is Rust executable located in the `/testclient` dir. It retries the api until it is available,
then counts the paid invoices, does a pay, then checks to see if it's greater than 2 (the initial number of paid invoices). It can be configured with:

- `MAX_RETRIES` (default: `30`)
- `API_URL` (default: `"http://localhost:8000"`)

### kubernetes configs

Both `antaeus` and `provider` have require similar Kubernetes yaml configs, so I use [`dhall`](https://dhall-lang.org/) to generate the configs for both. Check the `/deploy/dhall` dir for the source. You can generate the kubernetes yaml with:

```
make render-k8s-yaml-docker
```

which is pretty much just calling `dhall-to-yaml` serveral times, take a look in the `/Makefile` to see how exactly.

To edit the defined variables, update the file `./deploy/dhall/microservices.dhall`.

### Discussion bonus points

There's definitely a few things I could improve.

- installing microk8s in CI is slow/flakey.
- antaeus in docker seems to be making gradle network requests on startup. Should probably be packaged as a fat jar or something similar.
- testclient is flakey (can't tell the difference between a genuine network request/config error, and no updates happening because `provider` returned mostly `false`)
- current the CI docker builds tag images only as `latest`, which would make rollbacks impossible. I'd probably switch to a `git describe` tag format.
- there's currently only integration tests, no unit tests for the `provider` or `testclient`.
- both services also are deployed together, it would be better if they could release independently.

#### For a prod deployment...

- I'd maybe use similar tools to this: Dhall to generate k8s configs. I've checked the output k8s yaml into the repo here in a gitops style, but that'd get a bit more complicated when using real version numbers. Also Dhall gets a bit cumbersome with really complicated k8s configs (can see a bit of that in how I handled the different probes for the antaeus and provider services), so could maybe use something else like Kustomize or Helm.
- Expand the testclient to be like a post deploy smoke test, and roll back if it fails. And in production these might be real invoices with actual money, so the smoke test would need to behave accordingly.
- Proper CI job that does the production deploy after tests/staging envs have passed.

#### If a developers needs to push updates to just one of the services...

I'd use some sort of RBAC to grant permissions to just the required resources (`get`/`update`/etc. on `deployments`, `pods`), though I'd hope this is for testing and that actual deployments are being done by CI and not devs with `kubectl` access.

#### How do we prevent other services running in the cluster to talk to your service...

I'd probably use a [`network policy`](https://kubernetes.io/docs/concepts/services-networking/network-policies/) to limit the access to just antaeus. Some authorization could also be added, though that would be added complexity.
