# IDS721-02-food-recommend
## set up
1. create virtual enviroment
    python3 -m venv env
    source env/bin/activate
2. install rust
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    source "$HOME/.cargo/env"
3. create rust project
    cargo new path
    cargo build
4. write dockerfile and Makefile
    make format
5.  run web microservice
    cargo run

## project show

## deployment platform
### AWS APP Runner

### Use miniKube
1. deploy the docker
#my username is: sasays
#my project name is: randfood
    docker login -u username
    enter the password
    docker build . -t username/project
2. run minikube
#start the minikube
    minikube start
#view dashboard
    minikube dashboard --url
#create deployment and view it
    kubectl create deployment hi-minikube --image=registry.hub.docker.com/sasays/randfood
    kubectl get deployments
#deploy microserver and expose it
    kubectl expose deployment hi-minikube --type=LoadBalancer --port=8080
    kubectl get service hi-minikube
    minikube service hi-minikube  --url
#we can get the web url and access it via curl
    http://192.168.49.2:32470
    curl http://192.168.49.2:32470
#clean up
    kubectl delete service hi-minikube
    kubectl delete deployment hi-minikube
    minikube stop
##