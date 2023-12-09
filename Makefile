up:
	make styles & cargo watch -x "shuttle run"

psql-shuttle:
	docker exec -it shuttle_mili_shared_postgres psql -U postgres -h localhost -p 5432

styles:
	less-watch-compiler

docker:
	docker run -d -t -p 8089:5432 --name meme-pg -e POSTGRES_PASSWORD=postgres postgres

psql-docker:
	docker exec -it meme-pg psql -U postgres -h localhost -p 5432
