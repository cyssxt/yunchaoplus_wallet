import {PostgresConnectionOptions} from "typeorm/driver/postgres/PostgresConnectionOptions";

export interface Config{
    serverPort:number;
    db:PostgresConnectionOptions
}

const config:Config = {
    serverPort:8080,
    db:{
        type:"postgres",
        username: "postgres",
        host: "docker.for.mac.host.internal",
        port: 5432,
        database: "postgres",
        password: "123456",
        schema:'wallet',
        entities:["src/entity/*.ts","entity/*.js"]
    }
}

export default config;
