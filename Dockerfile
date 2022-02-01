FROM debian:buster-slim
RUN apt update
RUN apt install build-essential zlib1g-dev wget -y

RUN apt install curl -y

#Instalación de paquetes base para linux dev
RUN apt install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libspeechd-dev libxkbcommon-dev libssl-dev -y
RUN apt install libpq-dev -y #para usar el ORM Diesel
RUN apt install pkg-config -y

RUN rm -rf /var/lib/apt/lists/*

WORKDIR /home/process_files_in_ftp

#CMD ["cargo", "run start"]
ENTRYPOINT ["tail", "-f", "/dev/null"]
