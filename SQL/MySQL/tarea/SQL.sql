-- MySQL Script generated by MySQL Workbench
-- Mon 06 Mar 2023 09:56:54 -05
-- Model: New Model    Version: 1.0
-- MySQL Workbench Forward Engineering

SET @OLD_UNIQUE_CHECKS=@@UNIQUE_CHECKS, UNIQUE_CHECKS=0;
SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0;
SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='ONLY_FULL_GROUP_BY,STRICT_TRANS_TABLES,NO_ZERO_IN_DATE,NO_ZERO_DATE,ERROR_FOR_DIVISION_BY_ZERO,NO_ENGINE_SUBSTITUTION';

-- -----------------------------------------------------
-- Schema mydb
-- -----------------------------------------------------

-- -----------------------------------------------------
-- Schema mydb
-- -----------------------------------------------------
CREATE SCHEMA IF NOT EXISTS `mydb` DEFAULT CHARACTER SET utf8 ;
USE `mydb` ;

-- -----------------------------------------------------
-- Table `mydb`.`Usuarios`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `mydb`.`Usuarios` ;

CREATE TABLE IF NOT EXISTS `mydb`.`Usuarios` (
  `id_usuario` INT NOT NULL AUTO_INCREMENT,
  `nombre` VARCHAR(50) NOT NULL,
  `apellido` VARCHAR(50) NOT NULL,
  `correo_electronico` VARCHAR(100) NOT NULL,
  `contrasena` VARCHAR(100) NOT NULL,
  `fecha_registro` DATE NOT NULL,
  PRIMARY KEY (`id_usuario`));


-- -----------------------------------------------------
-- Table `mydb`.`Perfiles`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `mydb`.`Perfiles` ;

CREATE TABLE IF NOT EXISTS `mydb`.`Perfiles` (
  `id_perfil` INT NOT NULL AUTO_INCREMENT,
  `id_usuario` INT NOT NULL,
  `nombre_usuario` VARCHAR(50) NOT NULL,
  `imagen_perfil` VARCHAR(255) NULL DEFAULT NULL,
  `descripcion_perfil` TEXT NULL DEFAULT NULL,
  PRIMARY KEY (`id_perfil`),
  INDEX (`id_usuario` ASC) VISIBLE,
  CONSTRAINT ``
    FOREIGN KEY (`id_usuario`)
    REFERENCES `mydb`.`Usuarios` (`id_usuario`));


-- -----------------------------------------------------
-- Table `mydb`.`Juegos`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `mydb`.`Juegos` ;

CREATE TABLE IF NOT EXISTS `mydb`.`Juegos` (
  `id_juego` INT NOT NULL AUTO_INCREMENT,
  `titulo` VARCHAR(100) NOT NULL,
  `descripcion` TEXT NOT NULL,
  `desarrollador` VARCHAR(100) NOT NULL,
  `editor` VARCHAR(100) NOT NULL,
  `genero` VARCHAR(50) NOT NULL,
  `fecha_lanzamiento` DATE NOT NULL,
  `precio` DECIMAL(10,2) NOT NULL,
  PRIMARY KEY (`id_juego`));


-- -----------------------------------------------------
-- Table `mydb`.`Plataformas`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `mydb`.`Plataformas` ;

CREATE TABLE IF NOT EXISTS `mydb`.`Plataformas` (
  `id_plataforma` INT NOT NULL AUTO_INCREMENT,
  `nombre` VARCHAR(50) NOT NULL,
  `fabricante` VARCHAR(100) NOT NULL,
  `anio_lanzamiento` INT NOT NULL,
  PRIMARY KEY (`id_plataforma`));


-- -----------------------------------------------------
-- Table `mydb`.`Juegos_Plataformas`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `mydb`.`Juegos_Plataformas` ;

CREATE TABLE IF NOT EXISTS `mydb`.`Juegos_Plataformas` (
  `id_juego_plataforma` INT NOT NULL AUTO_INCREMENT,
  `id_juego` INT NOT NULL,
  `id_plataforma` INT NOT NULL,
  `fecha_lanzamiento` DATE NOT NULL,
  `precio` DECIMAL(10,2) NOT NULL,
  PRIMARY KEY (`id_juego_plataforma`),
  INDEX (`id_juego` ASC) VISIBLE,
  INDEX (`id_plataforma` ASC) VISIBLE,
  CONSTRAINT ``
    FOREIGN KEY (`id_juego`)
    REFERENCES `mydb`.`Juegos` (`id_juego`),
  CONSTRAINT ``
    FOREIGN KEY (`id_plataforma`)
    REFERENCES `mydb`.`Plataformas` (`id_plataforma`));


-- -----------------------------------------------------
-- Table `mydb`.`Clasificaciones`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `mydb`.`Clasificaciones` ;

CREATE TABLE IF NOT EXISTS `mydb`.`Clasificaciones` (
  `id_clasificacion` INT NOT NULL AUTO_INCREMENT,
  `nombre` VARCHAR(50) NOT NULL,
  `descripcion` TEXT NULL DEFAULT NULL,
  PRIMARY KEY (`id_clasificacion`));


-- -----------------------------------------------------
-- Table `mydb`.`Juegos_Clasificaciones`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `mydb`.`Juegos_Clasificaciones` ;

CREATE TABLE IF NOT EXISTS `mydb`.`Juegos_Clasificaciones` (
  `id_juego_clasificacion` INT NOT NULL AUTO_INCREMENT,
  `id_juego` INT NOT NULL,
  `id_clasificacion` INT NOT NULL,
  PRIMARY KEY (`id_juego_clasificacion`),
  INDEX (`id_juego` ASC) VISIBLE,
  INDEX (`id_clasificacion` ASC) VISIBLE,
  CONSTRAINT ``
    FOREIGN KEY (`id_juego`)
    REFERENCES `mydb`.`Juegos` (`id_juego`),
  CONSTRAINT ``
    FOREIGN KEY (`id_clasificacion`)
    REFERENCES `mydb`.`Clasificaciones` (`id_clasificacion`));


-- -----------------------------------------------------
-- Table `mydb`.`Comentarios`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `mydb`.`Comentarios` ;

CREATE TABLE IF NOT EXISTS `mydb`.`Comentarios` (
  `id_comentario` INT NOT NULL AUTO_INCREMENT,
  `id_usuario` INT NOT NULL,
  `id_juego` INT NOT NULL,
  `comentario` TEXT NOT NULL,
  `fecha_publicacion` DATE NOT NULL,
  PRIMARY KEY (`id_comentario`),
  INDEX (`id_usuario` ASC) VISIBLE,
  INDEX (`id_juego` ASC) VISIBLE,
  CONSTRAINT ``
    FOREIGN KEY (`id_usuario`)
    REFERENCES `mydb`.`Usuarios` (`id_usuario`),
  CONSTRAINT ``
    FOREIGN KEY (`id_juego`)
    REFERENCES `mydb`.`Juegos` (`id_juego`));


-- -----------------------------------------------------
-- Table `mydb`.`Valoraciones`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `mydb`.`Valoraciones` ;

CREATE TABLE IF NOT EXISTS `mydb`.`Valoraciones` (
  `id_valoracion` INT NOT NULL AUTO_INCREMENT,
  `id_usuario` INT NOT NULL,
  `id_juego` INT NOT NULL,
  `valoracion` INT NOT NULL,
  `fecha_valoracion` DATE NOT NULL,
  PRIMARY KEY (`id_valoracion`),
  INDEX (`id_usuario` ASC) VISIBLE,
  INDEX (`id_juego` ASC) VISIBLE,
  CONSTRAINT ``
    FOREIGN KEY (`id_usuario`)
    REFERENCES `mydb`.`Usuarios` (`id_usuario`),
  CONSTRAINT ``
    FOREIGN KEY (`id_juego`)
    REFERENCES `mydb`.`Juegos` (`id_juego`));


-- -----------------------------------------------------
-- Table `mydb`.`Compras`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `mydb`.`Compras` ;

CREATE TABLE IF NOT EXISTS `mydb`.`Compras` (
  `id_compra` INT NOT NULL AUTO_INCREMENT,
  `id_usuario` INT NOT NULL,
  `id_juego` INT NOT NULL,
  `fecha_compra` DATE NOT NULL,
  `precio` DECIMAL(10,2) NOT NULL,
  PRIMARY KEY (`id_compra`),
  INDEX (`id_usuario` ASC) VISIBLE,
  INDEX (`id_juego` ASC) VISIBLE,
  CONSTRAINT ``
    FOREIGN KEY (`id_usuario`)
    REFERENCES `mydb`.`Usuarios` (`id_usuario`),
  CONSTRAINT ``
    FOREIGN KEY (`id_juego`)
    REFERENCES `mydb`.`Juegos` (`id_juego`));


SET SQL_MODE=@OLD_SQL_MODE;
SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS;
SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS;