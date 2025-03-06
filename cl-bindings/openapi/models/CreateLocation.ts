/* tslint:disable */
/* eslint-disable */
/**
 * cl-backend
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

import { mapValues } from '../runtime';
import type { Environment } from './Environment';
import {
    EnvironmentFromJSON,
    EnvironmentFromJSONTyped,
    EnvironmentToJSON,
} from './Environment';

/**
 * 
 * @export
 * @interface CreateLocation
 */
export interface CreateLocation {
    /**
     * 
     * @type {string}
     * @memberof CreateLocation
     */
    author: string;
    /**
     * 
     * @type {string}
     * @memberof CreateLocation
     */
    description?: string | null;
    /**
     * 
     * @type {Environment}
     * @memberof CreateLocation
     */
    environment: Environment;
    /**
     * 
     * @type {string}
     * @memberof CreateLocation
     */
    name: string;
}



/**
 * Check if a given object implements the CreateLocation interface.
 */
export function instanceOfCreateLocation(value: object): value is CreateLocation {
    if (!('author' in value) || value['author'] === undefined) return false;
    if (!('environment' in value) || value['environment'] === undefined) return false;
    if (!('name' in value) || value['name'] === undefined) return false;
    return true;
}

export function CreateLocationFromJSON(json: any): CreateLocation {
    return CreateLocationFromJSONTyped(json, false);
}

export function CreateLocationFromJSONTyped(json: any, ignoreDiscriminator: boolean): CreateLocation {
    if (json == null) {
        return json;
    }
    return {
        
        'author': json['author'],
        'description': json['description'] == null ? undefined : json['description'],
        'environment': EnvironmentFromJSON(json['environment']),
        'name': json['name'],
    };
}

export function CreateLocationToJSON(value?: CreateLocation | null): any {
    if (value == null) {
        return value;
    }
    return {
        
        'author': value['author'],
        'description': value['description'],
        'environment': EnvironmentToJSON(value['environment']),
        'name': value['name'],
    };
}

