let f = ./functions.dhall

let antaeusInfo =
  { name = "antaeus"
  , port = 8000
  , image = "snazzybucket/antaeus"
  , tag = "latest"
  , healthPath = "/rest/health"
  , startupFailureThreshold = 30
  , env =
    [ { name = "PAYMENT_PROVIDER_ENDPOINT"
      , value = "http://provider/api/pay"
      }
    ]
  } : f.Info

let providerInfo =
  { name = "provider"
  , port = 8080
  , image = "snazzybucket/provider"
  , tag = "latest"
  , healthPath = "/health"
  , startupFailureThreshold = 3
  , env = [] : List { name : Text, value : Text }
  } : f.Info
in

{ antaeusInfo, providerInfo }
