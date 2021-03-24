let Info =
{ name : Text
, port : Natural
, image : Text
, tag : Text
, healthPath : Text
, startupFailureThreshold : Natural
, env : List { name : Text, value : Text }
}

let mkProbeBody = \(info : Info) ->
  { httpGet = { path = info.healthPath, port = info.port }
  , failureThreshold = 2
  , periodSeconds = 10
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
          , livenessProbe = mkProbeBody info
          , readinessProbe = mkProbeBody info
          , startupProbe = (mkProbeBody info) with failureThreshold = info.startupFailureThreshold
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
