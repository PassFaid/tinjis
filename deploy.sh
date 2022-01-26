#!/bin/bash

echo " "
sleep 2
echo " "
echo "Building Docker image faidonpassias/pleo-antaeus:latest"
docker build -t faidonpassias/pleo-antaeus:latest .
echo " "
sleep 2
echo " "
echo "Building Docker image for faidonpassias/pleo-payment:latest "
docker build -f ./payment/dockerfile -t faidonpassias/pleo-payment:latest .
echo " "
sleep 2
echo " "
cd kybernetes/
sleep 2
echo " "
echo "Applying kubernetes yaml files for deployments/services"
kubectl apply -f deployment-antaeus.yml
kubectl apply -f deployment-payment.yml
kubectl apply -f antaeus-service.yml
kubectl apply -f payment-service.yml
echo "Waiting 4 minutes for readiness checks for pleo-antaeus deployment, on the test runs the javalin server went up at approximately 3minutes on t2.medium machine"
sleep 2
echo "In order to find the antaeus-service nodeport please use this port"
kubectl get services | grep NodePort

