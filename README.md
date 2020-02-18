# Red Hat Openshift World Tour São Paulo - Demonstração II

## 1. Consiga o acesso ao seu cluster OpenShift!

## 2. Construindo uma aplicação conteinerizada em Rust (com Alpine Linux!)

  - APRESENTAÇÃO: Dockerfile base (Ubuntu 18.04 LTS + Musl) 
  
  - LINK: Imagem hospedada no repositório público no Dockerhub -- https://hub.docker.com/repository/docker/vmpereiraf/rust-musl-builder
  
  - APRESENTAÇÃO: Dockerfile final (Alpine + binário estático Rust -- miniaturizado!!!)
 
  - APRESENTAÇÃO: Código Hello World

  - OPCIONAL: Construindo e hospedando a imagem Docker base no IBM Container Registry (utilizando IBM CLI)
  ``ibmcloud cr build -t us.icr.io/vnderlev-namespace/rhos-worldtoursp-rustapp:latest ./``

  - OPCIONAL: Verificando por vulnerabilidades na imagem com a ferramenta security-advisor (utilizando IBM CLI) 
  ``ibmcloud cr vulnerability-assessment us.icr.io/vnderlev-namespace/rhos-worldtoursp-rustapp:latest``
  
## Criar uma toolchain no OpenShift (NOVO!)

  - CONFIGURAÇÕES: ``Develop a Kubernetes app``; Escolher ``Github``; existing; e a URL deste repositório (ou de seu fork); manter o checkbox em rastreamento de mudanças de código; Autorize o acesso ao Github.
  
  - ESPERAR DEPLOYMENT
  
  - CRIAR ROTA: Criar uma rota no projeto OpenShift
  
  - TESTE: Link
  
  - REALIZAR ALTERAÇÃO NO REPOSITÓRIO
  
  - VER MUDANÇAS AUTOMÁTICAS NA PIPELINE
