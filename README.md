Solution:

1)How would a new deployment look like for these services? What kind of tools would you use?
a)For Staging/ Production environments I would like to see a CI/CD pipeline to automate the process of building/deleting the ecosystem.
b)Also what i would strive for is high availability on both the pods (with more replicas) or app-based HA for example by mounting the database in a volume so there are no loss of data when the Pods get terminated.At the moment when the antaeus pod gets terminated the new pod has no recolection of the paid invoices. 
c)I would loadbalance the services by LoadBalancer service type.
d) I would protect the pods via Network Policies.


2) If a developers needs to push updates to just one of the services, how can we grant that permission without allowing the same developer to deploy any other services running in K8s?

I would setup RBAG in Kubernetes Cluster

3) How do we prevent other services running in the cluster to talk to your service. Only Antaeus should be able to do it.
First of all by running the service as ClusterIP so only cluster resources have access to the service and secondy,to tighten the perimeter more, I would use Network Policies.
Here is a sample Network Policy yaml file.

apiVersion: networking.k8s.io/v1
kind: NetworkPolicy
metadata:
  name: antaeus-payment-policy
  namespace: default
spec:
  podSelector:
    matchLabels:
      app: pleo-payment
  policyTypes:
  - Ingress
  ingress:
  - from:
    - podSelector:
        matchLabels:
          app: pleo-ataneus
    ports:
    - protocol: TCP
      port: 8000
