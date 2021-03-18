run:
	cargo run

test:
	curl -v localhost:8080/api/pay -X POST -H "Content-Type: application/json" -d '{ "currency": {"foo": "bar"}, "customer_id": 1, "value": 140.38 }'
