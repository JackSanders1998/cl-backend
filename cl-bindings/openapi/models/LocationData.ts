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
/**
 * 
 * @export
 * @interface LocationData
 */
export interface LocationData {
    /**
     * 
     * @type {Date}
     * @memberof LocationData
     */
    createdAt: Date;
    /**
     * 
     * @type {string}
     * @memberof LocationData
     */
    environment: string;
    /**
     * 
     * @type {string}
     * @memberof LocationData
     */
    locationId: string;
    /**
     * 
     * @type {string}
     * @memberof LocationData
     */
    name: string;
    /**
     * 
     * @type {Date}
     * @memberof LocationData
     */
    updatedAt: Date;
}

/**
 * Check if a given object implements the LocationData interface.
 */
export function instanceOfLocationData(value: object): value is LocationData {
    if (!('createdAt' in value) || value['createdAt'] === undefined) return false;
    if (!('environment' in value) || value['environment'] === undefined) return false;
    if (!('locationId' in value) || value['locationId'] === undefined) return false;
    if (!('name' in value) || value['name'] === undefined) return false;
    if (!('updatedAt' in value) || value['updatedAt'] === undefined) return false;
    return true;
}

export function LocationDataFromJSON(json: any): LocationData {
    return LocationDataFromJSONTyped(json, false);
}

export function LocationDataFromJSONTyped(json: any, ignoreDiscriminator: boolean): LocationData {
    if (json == null) {
        return json;
    }
    return {
        
        'createdAt': (new Date(json['created_at'])),
        'environment': json['environment'],
        'locationId': json['location_id'],
        'name': json['name'],
        'updatedAt': (new Date(json['updated_at'])),
    };
}

export function LocationDataToJSON(value?: LocationData | null): any {
    if (value == null) {
        return value;
    }
    return {
        
        'created_at': ((value['createdAt']).toISOString()),
        'environment': value['environment'],
        'location_id': value['locationId'],
        'name': value['name'],
        'updated_at': ((value['updatedAt']).toISOString()),
    };
}

