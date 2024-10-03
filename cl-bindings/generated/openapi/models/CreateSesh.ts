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
 * @interface CreateSesh
 */
export interface CreateSesh {
    /**
     * 
     * @type {string}
     * @memberof CreateSesh
     */
    locationId: string;
    /**
     * 
     * @type {string}
     * @memberof CreateSesh
     */
    notes?: string | null;
}

/**
 * Check if a given object implements the CreateSesh interface.
 */
export function instanceOfCreateSesh(value: object): value is CreateSesh {
    if (!('locationId' in value) || value['locationId'] === undefined) return false;
    return true;
}

export function CreateSeshFromJSON(json: any): CreateSesh {
    return CreateSeshFromJSONTyped(json, false);
}

export function CreateSeshFromJSONTyped(json: any, ignoreDiscriminator: boolean): CreateSesh {
    if (json == null) {
        return json;
    }
    return {
        
        'locationId': json['location_id'],
        'notes': json['notes'] == null ? undefined : json['notes'],
    };
}

export function CreateSeshToJSON(value?: CreateSesh | null): any {
    if (value == null) {
        return value;
    }
    return {
        
        'location_id': value['locationId'],
        'notes': value['notes'],
    };
}

