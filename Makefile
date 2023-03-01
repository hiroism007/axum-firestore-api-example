init:
	gcloud config set project mother-c7733
	gcloud auth configure-docker
	gcloud config set run/platform managed
	gcloud config set run/region us-west1

build:
	docker build -t rust-firestore-api .


tag:
	docker tag rust-firestore-api:latest gcr.io/mother-c7733/rust-firestore-api


push:
	docker push gcr.io/mother-c7733/rust-firestore-api


deploy:
	gcloud run deploy rust-firestore-api --image gcr.io/mother-c7733/rust-firestore-api --platform managed --region us-west1 \
  	--update-env-vars PROJECT_ID=mother-c7733,RUST_BACKTRACE=1 \
	--allow-unauthenticated --max-instances 1 --min-instances 1

go:
	make build
	make tag
	make push
	make deploy

cloud-build:
	 gcloud builds submit --region global
