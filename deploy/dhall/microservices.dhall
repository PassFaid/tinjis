let f = ./functions.dhall

let antaeusInfo =
  { name = "antaeus"
  , port = 8000
  , image = "snazzybucket/antaeus"
  , tag = "latest"
  , healthPath = "/rest/health"
  , initialDelaySeconds = 120
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
  , initialDelaySeconds = 30
  , env = [] : List { name : Text, value : Text }
  } : f.Info
in

{ antaeusInfo, providerInfo }
