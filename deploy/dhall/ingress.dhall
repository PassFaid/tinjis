{ apiVersion = "networking.k8s.io/v1beta1"
, kind = "Ingress"
, metadata.name = "antaeus"
, spec.rules
  =
  [ { http.paths
      =
      [ { backend = { serviceName = "antaeus", servicePort = 80 }, path = "/" }
      ]
    }
  ]
}
