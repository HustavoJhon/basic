FROM php:8.2-apache

# Instalar extensiones necesarias
RUN docker-php-ext-install pdo pdo_mysql mysqli

# Habilitar mod_rewrite para Apache
RUN a2enmod rewrite

# Configurar DocumentRoot
RUN sed -i 's|/var/www/html|/var/www/html|g' /etc/apache2/sites-available/000-default.conf

# Copiar archivos del proyecto
COPY . /var/www/html/

# Dar permisos
RUN chown -R www-data:www-data /var/www/html

# Exposer puerto
EXPOSE 80

CMD ["apache2-foreground"]
