let Info =
{ name : Text
, port : Natural
, image : Text
, tag : Text
, healthPath : Text
, initialDelaySeconds : Natural
, env : List { name : Text, value : Text }
}

let mkDeployment = \(info : Info) ->
  { apiVersion = "apps/v1"
  , kind = "Deployment"
  , metadata = { labels.app = info.name, name = info.name ++ "-deployment" }
  , spec =
    { selector.matchLabels.app = info.name
    , template =
      { metadata.labels.app = info.name
      , spec.containers
        =
        [ { env = info.env
          , image = "${info.image}:${info.tag}"
          , livenessProbe =
            { httpGet = { path = info.healthPath, port = info.port }
            , initialDelaySeconds = info.initialDelaySeconds
            , periodSeconds = 30
            }
          , name = info.name
          , ports = [ { containerPort = info.port } ]
          }
        ]
      }
    }
  }

let mkService = \(info : Info) ->
  { apiVersion = "v1"
  , kind = "Service"
  , metadata = { labels.app = info.name, name = info.name }
  , spec =
    { ports = [ { port = 80, protocol = "TCP", targetPort = info.port } ]
    , selector.app = info.name
    }
  }

in
{ mkDeployment, mkService, Info }
