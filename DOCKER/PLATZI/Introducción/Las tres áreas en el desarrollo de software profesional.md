_"Docker te permite construir, distribuir y ejecutar cualquier aplicación en cualquier lado."_

**Problemáticas del desarrollo de software**

**1. Construir -** Escribir código en la máquina del desarrollador. (Compile, que no compile, arreglar el bug, compartir código, etc. )

**Problemática:**

- Entorno de desarrollo (paquetes)
- Dependencias (Frameworks, bibliotecas)
- Versiones de entornos de ejecución (runtime, versión Node)
- Equivalencia de entornos de desarrollo (compartir el código)
- Equivalencia con entornos productivos (pasar a producción)
- Servicios externos (integración con otros servicios ejem: base de datos)

**2. Distribuir** - Llevar la aplicación donde se va a desplegar (Transformarse en un artefacto)

**Problemática:**

- Output de build heterogeo (múltiples compilaciones)
- Acceso a servidores productivos (No tenemos acceso al servidor)
- Ejecución nativa vs virtualizada
- Entornos Serverless

**3. Ejecutar** - Implementar la solución en el ambiente de producción (Subir a producción)  
El reto Hacer que funcione como debería funcionar

**Problemática:**

- Dependencia de aplicación (paquetes, runtime)
- Compatibilidad con el entorno productivo (sistema operativo poco amigable con la solución)
- Disponibilidad de servicios externos (Acceso a los servicios externos)
- Recursos de hardware (Capacidad de ejecución - Menos memoria, procesador más debil)