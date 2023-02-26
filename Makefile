init:
	gcloud config set project playground-hiro
	gcloud auth configure-docker
	gcloud config set run/platform managed
	gcloud config set run/region us-west1

build:
	docker build --platform linux/amd64 -t rust-firestore-api .


tag:
	docker tag rust-firestore-api:latest gcr.io/playground-hiro/rust-firestore-api


push:
	docker push gcr.io/playground-hiro/rust-firestore-api


deploy:
	gcloud run deploy rust-firestore-api --image gcr.io/playground-hiro/rust-firestore-api --platform managed --region us-west1 \
  	--update-env-vars GOOGLE_APPLICATION_CREDENTIALS=/etc/secrets/firebase.json \
	--update-secrets /etc/secrets/firebase.json=TEST_FIREBASE_SECRET_JSON:latest \
	--allow-unauthenticated --max-instances 1 --min-instances 1

go:
	make build
	make tag
	make push
	make deploy
