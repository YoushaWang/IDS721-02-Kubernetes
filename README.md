# IDS721-02-food-recommend
## Intro
In this project, we'r going to build a functional web MicroService in Rust and deploy it on Kubernetes.    
My project will randomly recommend a popular dish for customer.
## Set up
1. create virtual enviroment
```
    python3 -m venv env
    source env/bin/activate
```
2. install rust
```
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    source "$HOME/.cargo/env"
```
3. create rust a new project
```
    cargo new path
    cargo build
```
4. write dockerfile and Makefile and run make to check errors
```
    make format
```
5.  run web microservice
```
    cargo run
```
## project display
1. in the "/", show the topic of the project
<img width="600" alt="show1" src="/img/show_index.png">

2. in the "/food", a dish name is recommended for customer
<img width="600" alt="show2" src="/img/show_food.png">

## deployment platform
### AWS 
1. use git clone to add code to AWS cloud9
2. install virtual enviroment and rust as written before
3. in the venv, run the project to check the correctness
<img width="600" alt="C9" src="/img/C9.png">    

4. go to AWS ECR and create a private repository, named it as randfood   
5. in the push command inside the repository, type all code in Cloud9 terminal
```
aws ecr get-login-password --region us-east-1 | docker login --username AWS --password-stdin 125881730842.dkr.ecr.us-east-1.amazonaws.com
docker build -t randfood .
docker tag randfood:latest 125881730842.dkr.ecr.us-east-1.amazonaws.com/randfood:latest
docker push 125881730842.dkr.ecr.us-east-1.amazonaws.com/randfood:latest
```
<img width="600" alt="ECR" src="/img/ECR.png">    
<img width="600" alt="EndC9" src="/img/endofC9.png">   

6. go to AWS app runner, import the image of ECR into app runner, deploy the app
7. after deploy success, click the url then we can see our project
```
https://m7gpmc5m3p.us-east-1.awsapprunner.com/
```
<img width="600" alt="APP" src="/img/APP.png">

### Use miniKube
1. deploy the docker
* my username is: sasays
* my project name is: randfood
```
    docker login -u username
    enter the password
    docker build . -t username/project
```
<img width="600" alt="LI" src="/img/LI.png">
<img width="600" alt="build" src="/img/D_build.png">

2. run minikube
* start the minikube
```
    minikube start
```
<img width="600" alt="start" src="/img/kubeStart.png">

* view dashboard
```
    minikube dashboard --url
```
* create deployment and view it
```
    kubectl create deployment hi-minikube --image=registry.hub.docker.com/sasays/randfood
    kubectl get deployments
```
* deploy microserver and expose it
```
    kubectl expose deployment hi-minikube --type=LoadBalancer --port=8080
    kubectl get service hi-minikube
    minikube service hi-minikube  --url
```
<img width="600" alt="kube" src="/img/kube.png">

* we can get the web url and access it via curl
```
    http://192.168.49.2:32470
    curl http://192.168.49.2:32470
```
it will return 
```
Hello, random dish recommendation for you today!
```
* clean up
```
    kubectl delete service hi-minikube
    kubectl delete deployment hi-minikube
    minikube stop
```
<img width="600" alt="stop" src="/img/kubeStop.png">

##