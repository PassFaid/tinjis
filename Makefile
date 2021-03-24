render-k8s-yaml:
	dhall-to-yaml --generated-comment --documents --file deploy/dhall/deployment.dhall > deploy/generated.yaml
	dhall-to-yaml --documents --file deploy/dhall/service.dhall >> deploy/generated.yaml
	dhall-to-yaml --documents --file deploy/dhall/ingress.dhall >> deploy/generated.yaml
	echo 'output written to `./deploy/generated.yaml`'

DHALL_IN_DOCKER := \
	@docker run --rm \
		--volume $(CURDIR):/data \
		--workdir /data \
		dhallhaskell/dhall-json dhall-to-yaml \

render-k8s-yaml-docker:
	$(DHALL_IN_DOCKER) --generated-comment --documents --file deploy/dhall/deployment.dhall > deploy/generated.yaml
	$(DHALL_IN_DOCKER) --documents --file deploy/dhall/service.dhall >> deploy/generated.yaml
	$(DHALL_IN_DOCKER) --documents --file deploy/dhall/ingress.dhall >> deploy/generated.yaml
	@echo 'output written to `./deploy/generated.yaml`'

docker-build-antaeus:
	docker build . -t snazzybucket/antaeus:latest

docker-build-provider:
	docker build ./provider --file ./provider/Dockerfile --tag snazzybucket/provider:latest

docker-build-testclient:
	docker build ./testclient --file ./testclient/Dockerfile --tag snazzybucket/testclient:latest

.PHONY: deploy
deploy:
	kubectl apply -f deploy/generated.yaml
