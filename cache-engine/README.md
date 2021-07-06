# Polkamusic Cache Engine
This application listen to the events on the blockchain and produces a cache of Polkamusic data in a mysql database.
The database can be used for example, for an  explorer app or for statistic analysis.

## Requirements

The following instructions refere to the installation on a Linux/Debian 10:

Install [yarn](https://yarnpkg.com/)  
Install [NodeJS](https://nodejs.org)  
from the CLI:  
```sh
yarn add @polkadot/api
yarn add moment
yarn add mysql
apt-get install mysql
```

## Create Database and grant access
launch the mysql cli:
```sh
mysql
```
and copy/paste: 
```
create database polkamusic;  
CREATE USER 'polkamusic'@'localhost' IDENTIFIED BY 'aszxqw1234';  
GRANT ALL ON polkamusic.* TO polkamusic@'localhost';  
flush privileges;  
```
replacecing the password with your preferred one.

## Change Password
Replace the password in the file polkamusic-cache-engine.js
```
let connection = mysql.createConnection({
        host     : '127.0.0.1',
        user     : 'polkamusic',
        password : 'aszxqw1234'         // change to your current password, this is only a sample
    });
```
## Launch the Cache Engine:
```
node polkamusic-cache-engine.js
```

