render-k8s-yaml:
	dhall-to-yaml --generated-comment --documents --file deploy/dhall/deployment.dhall
	dhall-to-yaml --generated-comment --documents --file deploy/dhall/service.dhall
