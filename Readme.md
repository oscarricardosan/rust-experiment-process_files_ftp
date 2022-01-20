# Importante

## Levantar entorno de depuración/desarrollo

El entorno de depuración cuenta con **cargo** instalado dentro del contenedor, el cual 
permitirá depurar y compilar nuestra aplicación Rust.

``` bash
cd ./app_rust

#Levantar contenedor docker en primer plano
docker-compose -f docker-compose-debug.yml up 

#Acceder a contenedor
docker exec -it app-rust-to-ftp bash

#Comandos utiles
cd ./home/process_files_in_ftp/
cargo run start
cargo build --release
mv target/release/process_files_in_ftp target/process_files_in_ftp

```



## Levantar entorno de producción

El entorno de producción _no cuenta_ con **cargo** instalado dentro del contenedor, esta 
configurado solo para permitir la ejecución de la aplicación.

Es importante construir la aplicación en entorno de debug antes de correr este entorno.

``` bash
cd ./app_rust

#Levantar contenedor docker en primer plano
docker-compose -f docker-compose.yml up 

#Acceder a contenedor
docker exec -it app-rust-to-ftp bash

./target/process_files_in_ftp --help
```



## Subir código a servidor

``` bash
cd ./app_rust

#Levantar contenedor docker en primer plano
docker-compose -f docker-compose-debug.yml up 

#Acceder a contenedor
docker exec -it app-rust-to-ftp bash

#Generar aplicación
cargo build --release
mv target/release/process_files_in_ftp target/process_files_in_ftp

#Eliminar archivos no necesarios
rm -rf target/debug
rm -rf target/release/
rm -rf tmp/*.jpg

#Detener contenedor
exit

#Copiar archivos a servidor:
scp -r app_rust root@xxx.xxx.xxx.xxx:/home/process_files

#ingresar a servidor
ssh root@xxx.xxx.xxx.xxx

cd /home/process_files/app_rust

#En otra pestaña:
cd /home/process_files/app_rust/
docker-compose -f docker-compose.yml up 
docker exec -it app-rust-to-ftp bash


```


## Base de datos

``` bash
#Conectar a contenedor
docker exec -it app-rust-ftp-postgres bash

#Ingresar a BD
psql -U savne -d ftp

```
