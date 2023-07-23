# rsquid

Squid Helper Package for R

## Installation

install from github

```R
R> remotes::install_github("haitusense/rsquid@20230721")
```

install from local

```R
R> install.packages("../rsquid/", repos = NULL, type = "source")
```

## Usage

### plot

```R
library(jsonlite)
library(ggplot2)
library(svglite)
library(readr)

p <- ggplot(iris, aes(x = Sepal.Length, y = Sepal.Width)) + geom_point()
ggsave("temp.svg", plot = p)
svg_string <- read_file("temp.svg")
dst <- list(id = "plot_svg", value = svg_string)
rsquid::named_pipe("NamedPipe", toJSON(dst))
file.remove("temp.svg")

# in js
# const svgElement = document.getElementsByClassName('svglite');
# svgElement[0].removeAttribute('width');
# svgElement[0].removeAttribute('height');
# svgElement[0].setAttribute('position', "absolute");
# svgElement[0].setAttribute('top', "0");
# svgElement[0].setAttribute('left', "0");
# svgElement[0].setAttribute('height', "100%");
```

```R
library(jsonlite)
library(ggplot2)
library(svglite)
library(XML)

p <- ggplot(iris, aes(x = Sepal.Length, y = Sepal.Width)) + geom_point()
ggsave("temp.svg", plot = p)
svg <- xmlInternalTreeParse("temp.svg")
file.remove("temp.svg")

svg <- xmlRoot(svg)
svg <- XML::removeAttributes(svg, "width")
svg <- XML::addAttributes(svg, "height" = "100%")
svg_string <- saveXML(svg)
```

```R
library(ggplot2)
library(gridSVG)
library(XML)
ggplot(iris, aes(x = Sepal.Length, y = Sepal.Width)) + geom_point()
svg <- grid.export(NULL, indent = F, prefix ="gridsvg")$svg
svg <- XML::removeAttributes(svg, "width")
svg <- XML::addAttributes(svg, "height" = "100%")
svg <- XML::removeChildren(svg, "metadata")
svg_string <- saveXML(svg)

dst <- list(id = "plot_svg", value = svg_string)
rsquid::named_pipe("NamedPipe", toJSON(dst))
```

### MemoryMappedFile

```R
src <- c(1, 2, 3, 4)
rsquid::writeMemoryMappedFile("mmf", as.integer(a))
dst <- rsquid::readMemoryMappedFile("mmf")

src <- c(1.1, 2.4, 3.6, 4.8)
rsquid::writeMemoryMappedFileFloat("mmf", a)
dst <-rsquid::readMemoryMappedFileFloat("mmf")
```