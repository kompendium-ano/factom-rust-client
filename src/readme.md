# Client modules

The Factom-Client is seperated into logical modules. 

## Address

Relating to Address functions

## Api

The main api client module holding the Factom struct from hich requests are constructed

## Balance

For balance related functions.

## Block

For functions dealing with block data queries.

## Chain

For functions handling chain data.

## Compose

Functions that compose transactions, entries and identities.

## Constants

Static fields for use.

## Debug

Factomd debug functions. Tests exist but are disabled for this module and require running a local factomd node.

## Entry

For querying entires.

## Factomd

General functions relating to factomd

## Generate

Functions for generating addresses or identities.

## Identity

Relating to identity functions.

## Import

For importing addresses or identities

## Requests

Request handling functions intrinsic to the factom struct

## Responses

Response handling functions to parse json responses into objects

## Tx

Functions relating to transactions

## Walletd

General utility functions relating to factom-walletd