render-k8s-yaml:
	dhall-to-yaml --generated-comment --documents --file deploy/dhall/deployment.dhall
	dhall-to-yaml --generated-comment --documents --file deploy/dhall/service.dhall
	dhall-to-yaml --generated-comment --documents --file deploy/dhall/ingress.dhall

DHALL_IN_DOCKER := \
	@docker run --rm \
		--volume $(CURDIR):/data \
		--workdir /data \
		dhallhaskell/dhall-json dhall-to-yaml \

render-k8s-yaml-docker:
	$(DHALL_IN_DOCKER) --generated-comment --documents --file deploy/dhall/deployment.dhall
	$(DHALL_IN_DOCKER) --generated-comment --documents --file deploy/dhall/service.dhall
	$(DHALL_IN_DOCKER) --generated-comment --documents --file deploy/dhall/ingress.dhall
