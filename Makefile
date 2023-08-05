
integration:
	./scripts/integration.sh

pg_start:
	./scripts/init_db.sh

pg_migrate:
	SKIP_DOCKER=true $(MAKE) pg_start

pg_stop:
	./scripts/stop_db.sh

test:
	$(MAKE) pg_start
	cargo test
	$(MAKE) pg_stop

ci:
	$(MAKE) pg_stop
	echo "Start Postgres"
	$(MAKE) pg_start
	echo "Tests"
	cargo test
	# echo "Tests w/ Coverage"
	# cargo tarpaulin --ignore-tests
	echo "Stop Postgres"
	echo "Lint"
	cargo clippy -- -D warnings
	echo "Format"
	cargo fmt -- --check
	echo "Audit"
	cargo audit
	# open ../coverage/tarpaulin-report.html
	$(MAKE) pg_stop

cert:
	openssl req -x509 -newkey rsa:4096 -nodes -keyout secrets/ssl/key.pem -out secrets/ssl/cert.pem -days 365 -subj '/CN=localhost'

