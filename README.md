Comandos para usar la API

ejecutamos el server con cargo run

y aparecera tipo "servidor disponible en http: x 

verificar que la API funciona:

curl http: x

Respuesta:

API funcionando

Obtener todos los usuarios (en bash)

curl http (los numeros que te salieron donde esta activo)/users

crea un usuario (bash)

curl -X POST http /users \
-H "Content-Type: application/json" \
-d '{"name":"Manolo"}'

eliminar usuario por ID

curl -X DELETE http /users/1

