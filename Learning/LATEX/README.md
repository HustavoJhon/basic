### 1.1 ¿Qué es TEX?
TEXes un sistema de composición de documentos de alta calidad, orientado especialmente
a la creación de documentos científicos y técnicos que incluyen fórmulas matemáticas.
Fue creado por Donald Knuth en 1978.
A diferencia de un procesador de textos como por ejemplo Microsoft Word, TEXno es
una aplicación sino un lenguaje de programación que require compilar el código fuente
para obtener el documento final. Esto, que a priori podría parecer una desventaja, en
realidad es la gran ventaja de TEXfrente a los procesadores de texto que siguen el para-
digma WYSIWYG (What You See Is What You Get), ya que permite separar fácilmente
el contenido y la estructura de un documento, de su formato, de manera que el usua-
rio puede centrarse en el contenido y la estructura del documento, y dejar que TEXse
encargue del formato. De hecho, TEXincorpora un potente lenguaje de marcado para es-
tructurar y formatear el texto de un documento. Por ejemplo, mientras que para poner
una palabra en negrita con un procesador de textos como Microsoft Word, bastaría con
seleccionar la palabra y hacer clic en el botón de negrita para ver automáticamente la
palabra en negrita en la pantalla del ordenador, en TEXhabría que escribir en el fichero
con el código fuente {\bf palabra} y después compilar el código fuente para obtener
un documento final con la palabra en negrita (el comando \bf, que permite aplicar la
negrita, se conoce como marca o tag en inglés.)

### 1.2 ¿Qué es LATEX?

Tanto TExcom LATEX son programas de codigo abierto, liberados bajo la licencia LPPPL.
Otra de las grandes ventajas de LATEX es que existen multitud de paquetes de codigo libre para generar distintos tipos de documentos que pueden descargarse desde el respositorio CRAN.

### 1.3 Instalacion 

Existen distintas distribuciones de LATEXy algunas de ellas son multiplataforma, es decir,
están disponibles para diferentes sistemas operativos. Las distribuciones más comunes
son:
• TeXLive para Windows, Mac OSX y Linux.
• MiKTeX para Windows, Mac OSX y Linux.
• MacTeX para Mac OSX.
En sus respectivas páginas está explicado el procedimiento de instalación de cada una.
Junto a la distribución de LATEXes también habitual instalar algún editor de texto para
escribir el código fuente. En realidad puede usarse cualquier editor de texto que ya esté
instalado en nuestro sistema operativo, pero los existen entornos de edición especializados
que facilitan muchas de las tareas del proceso de composición de documentos con LATEX.
Los más comunes son:
• TexMaker. Es un editor de texto libre, multiplataforma, con muchos asistentes
disponibles que permite previsualizar en tiempo real el documento final en pdf
• Texstudio. Es otro editor libre y multiplataforma que incorpora aún más asistentes
que el anterior.
• Vim Es un editor de texto simple de propósito general que también es libre y
multiplataforma. Incorpora paquetes o plugins específicos para facilitar la creación
de documentos con LATEX. Especialmente indicado para trabajar desde la terminal.
• Emacs. Es otro editor similar a Vim, muy extendido entre los usuarios que prefieren
usar la terminal.
• Visual Studio Code. Es un potente entorno de desarrollo multipropósito. Dispone
de paquetes para los lenguajes de programación más comunes, entre ellos LATEX.
Pero también se puede empezar a componer documentos sin necesidad de instalar nada
en el ordenador, usando un editor on-line como por ejemplo Overleaf

### 1.4 Hola LaTeX

A modo de ejemplo, empezamos por crear un sencillo documento con el texto "Hola LaTeX".

Para ello utilizamos nuestro editor de texto preferido para crear un fichero de texto con el nombre main.tex y el siguiente contenido:

```tex
\documentclass{article}
\usepackage[spanish]{babel}
\begin{document}
Hola \LaTeX
\end{document}
```

### 1.5 Compilacion

Cada distribucion de LaTeX viene con varios compiladores. Los mas habituales son:

- latex: Es el compilador mas antiguo y genera documentos en `formato dvi`, que es un formato independiente creado mucho antes que el formato `pdf`.
- `pdflatex`: Es un compiladore mas usado y genera documentos en formato **pdf**
- `xelatex`: Es un compiladore mas moderno que admite caracteres Unicode en el codigo fuente y el uso de tipograficas mas modernas.

En una terminal la compilacion de este documento seria fecleando el comando `latex main.tex, pdflatex main.tex o xelatex main.tex`, dependiendo del compilador que se quiera usar.

```tex
pdflatex main.tex
```

> ? Advertencia: Si el documento contiene referencias cruzadas, citaciones bibliograficas, tabla de contenidos o indices, es necesario compilar el documento dos o tres veces para que se generen automaticamente esas partes.


