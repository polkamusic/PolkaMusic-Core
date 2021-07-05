# Polkamusic Cache Engine
This application listen to the events on the blockchain and produces a cache pf Polkamusic data in a mysql database.
The database can be used from an explorer app or for statistic analysis.

## Requirements

Install the libraries as follows:

yarn add @polkadot/api
yarn add moment
yarn add mysql
apt-get install mysql


## Create Database and grant access
from mysql cli:  
create database polkamusic;  
CREATE USER 'polkamusic'@'localhost' IDENTIFIED BY 'aszxqw1234';  
GRANT ALL ON polkamusic.* TO polkamusic@'localhost';  
flush privileges;  

## Change Password

(draft to be completed)


