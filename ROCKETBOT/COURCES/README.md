# Flask en Azure Kubernetes

> https://github.com/ital08/python-flask-demo-app

[![PYTHON](https://img.shields.io/badge/python-black?style=for-the-badge&logo=python)](https://github.com/hustavojhon) [![DOCKER](https://img.shields.io/badge/docker-black?style=for-the-badge&logo=docker)](https://github.com/hustavojhon) [![AZURE](https://img.shields.io/badge/Azure-black?style=for-the-badge&logo=microsoft-azure)](https://github.com/hustavojhon)

Desplegar una aplicación Flask en Azure Kubernetes Service (AKS) utilizando secretos almacenados en Kubernetes incluye procesos como la construcción de una imagen Docker, su almacenamiento en Azure Container Registry (ACR) y la creación y configuración de un clúster AKS. Las ventajas que ofrece esto son la escalabilidad y gestión simplificada, la seguridad mejorada al utilizar secretos de Kubernetes para gestionar información sensible, y el monitoreo detallado con herramientas como Azure Monitor y Prometheus. AKS permite escalar automáticamente las aplicaciones según la demanda y facilita la recuperación ante fallos. Para este despliegue se utilizan diversas herramientas como Azure CLI para crear y gestionar recursos de Azure, Docker para construir, etiquetar y subir imágenes de contenedores a ACR, y Kubernetes para orquestar los contenedores y servicios. Azure Monitor se utiliza para supervisar el rendimiento del clúster, mientras que Prometheus y Grafana ofrecen monitoreo avanzado y visualización de métricas con dashboards.
#### Uso de Herramientas
1. **Azure CLI**: Utilizado para crear y gestionar recursos de Azure, como grupos de recursos, registros de contenedores y clústeres de AKS.
2. **Docker**: Herramienta esencial para construir, etiquetar y subir imágenes de contenedores a ACR.
3. **Kubernetes**: Orquesta los contenedores y servicios, manejando el despliegue, escalado y operación de las aplicaciones.
4. **Azure Container Registry (ACR)**: Almacena y gestiona las imágenes de contenedores, facilitando su integración con AKS.
5. **Azure Monitor**: Supervisión avanzada que recopila métricas y registros, proporcionando análisis y alertas para gestionar el rendimiento del clúster.
6. **Prometheus y Grafana**: Herramientas de código abierto para exportar y visualizar métricas, ofreciendo paneles detallados de monitoreo.

- **Kubernetes Secrets**: Almacenan y gestionan información sensible como credenciales, que se integran en los contenedores mediante configuraciones en los archivos YAML de despliegue.
- **ConfigMaps y Secrets**: Separan configuraciones sensibles y no sensibles, mejorando la seguridad y organización de los recursos.
#### Arquitectura del Despliegue
1. **Construcción de la Imagen Docker**: Crear una imagen del aplicación Flask y se sube a ACR.
2. **Creación del Clúster de AKS**: Se configura un clúster de AKS con nodos y se habilitan complementos como Azure Monitor.
3. **Configuración de Despliegue con YAML**: Archivos YAML definen los recursos necesarios (Deployments y Services) y especifican cómo los contenedores deben utilizar los secretos.
4. **Monitoreo y Gestión**: Azure Monitor y otros como Prometheus y Grafana se utilizan para monitorear el rendimiento y asegurar el funcionamiento correcto de la aplicación.
