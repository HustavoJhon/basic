#!/bin/bash

# Script para iniciar el proyecto con Docker

echo "🚀 Iniciando Zay Shop con Docker..."
echo ""

# Verificar si Docker está disponible
if ! command -v docker &> /dev/null; then
    echo "❌ Docker no está instalado. Por favor instala Docker."
    exit 1
fi

echo "📦 Construyendo contenedores..."
docker compose up -d --build

if [ $? -eq 0 ]; then
    echo ""
    echo "✅ Contenedores iniciados exitosamente!"
    echo ""
    echo "🌐 Accede a la aplicación en:"
    echo "   http://localhost:8080"
    echo ""
    echo "🔐 Credenciales:"
    echo "   Usuario: admin"
    echo "   Contraseña: 123456"
    echo ""
    echo "📊 Panel Admin:"
    echo "   http://localhost:8080/admin/"
    echo ""
    echo "💡 Para ver los logs:"
    echo "   docker compose logs -f"
    echo ""
    echo "⏹️  Para detener:"
    echo "   docker compose down"
else
    echo "❌ Error al iniciar los contenedores"
    exit 1
fi
