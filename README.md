# default-service
Respond to all incoming HTTP requests with a message. 

Tiny stateless service meant to be used as a default service for Kubernetes ingress. Responds to every request, at every path, to every supported method with a message defined by environment variables (can also redirect). 

### Configuration

There are five environment variables used for configuration:    

| Variable      | Values        | Default  | Required |
| ------------- |:-------------:|:--------:|:--------:|
| `PORT` | 1-65535 | 80 | No |
| `STATUS_CODE` | HTTP Status code | 404 | No |
| `LOG` | `true`/`false` | true | No |
| `ACTION_TYPE` | `XML`/`JSON`/`Text`/`Redirect` | `Text` | No |
| `ACTION_DATA` | Content to send |  | Yes |     
    

For example, to send an XML response you would configure the environment variables as the following:
| Variable | Value  |
|----------|:------:|
| `ACTION_TYPE` | `XML` |
| `ACTION_DATA` | `"<Error><Message>Hello there!</Message></Error>"` |

For action type `Redirect`, `ACTION_DATA` represents the URI to redirect to.

#### `ACTION_DATA` string replacements

Before sending off the response, the following strings get replaced:
* `$HOST` gets replaced with the request's host header
* `$PATH` gets replaced with the request's path

### Docker

The docker image is `weilbyte/default-service`, use `latest` tag.
Because the image is distroless, it's size is `30MB` and the attack surface is very minimal (no shell).

### Configuring ingress

First, you need to deploy the container and create a service that points to it (port 80).    
  
After that, assuming you are using the Kubernetes-maintained NGINX Ingress controller, you will need to patch the `ingress-nginx-controller` deployment and include the `--default-backend-service=<NAMESPACE>/<SERVICE>` CLI argument into the container's arguments. Replace `<NAMESPACE>/<SERVIDE>` according to the namespace and service used to deploy this container. 

Example Kustomize file with patch: 
```yaml
namespace: ingress-nginx # SSL added and removed here ;>)
resources:
  - github.com/kubernetes/ingress-nginx//deploy/static/provider/baremetal
patches:
  - target:
      kind: Deployment
      name: ingress-nginx-controller
    patch: |-
      - op: add
        path: /spec/template/spec/containers/0/args/-
        value: --default-backend-service=default-service-namespace/default-service-service
```

After that, your `default-service` deployment will catch any request not matching any ingress hostnames.
