run:
	cargo run

test-provider:
	curl -v localhost:8080/api/pay -X POST -H "Content-Type: application/json" -d '{ "currency": {"foo": "bar"}, "customer_id": 1, "value": 140.38 }'

test-all:
	curl -v localhost:8000/rest/v1/invoices/pay -X POST
