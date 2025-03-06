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
import type { Location } from './Location';
import {
    LocationFromJSON,
    LocationFromJSONTyped,
    LocationToJSON,
} from './Location';

/**
 * 
 * @export
 * @interface SeshWithLocation
 */
export interface SeshWithLocation {
    /**
     * 
     * @type {Date}
     * @memberof SeshWithLocation
     */
    createdAt: Date;
    /**
     * 
     * @type {Date}
     * @memberof SeshWithLocation
     */
    end?: Date | null;
    /**
     * 
     * @type {Location}
     * @memberof SeshWithLocation
     */
    location: Location;
    /**
     * 
     * @type {string}
     * @memberof SeshWithLocation
     */
    notes?: string | null;
    /**
     * 
     * @type {string}
     * @memberof SeshWithLocation
     */
    seshId: string;
    /**
     * 
     * @type {Date}
     * @memberof SeshWithLocation
     */
    start: Date;
    /**
     * 
     * @type {Date}
     * @memberof SeshWithLocation
     */
    updatedAt: Date;
    /**
     * 
     * @type {string}
     * @memberof SeshWithLocation
     */
    userId: string;
}

/**
 * Check if a given object implements the SeshWithLocation interface.
 */
export function instanceOfSeshWithLocation(value: object): value is SeshWithLocation {
    if (!('createdAt' in value) || value['createdAt'] === undefined) return false;
    if (!('location' in value) || value['location'] === undefined) return false;
    if (!('seshId' in value) || value['seshId'] === undefined) return false;
    if (!('start' in value) || value['start'] === undefined) return false;
    if (!('updatedAt' in value) || value['updatedAt'] === undefined) return false;
    if (!('userId' in value) || value['userId'] === undefined) return false;
    return true;
}

export function SeshWithLocationFromJSON(json: any): SeshWithLocation {
    return SeshWithLocationFromJSONTyped(json, false);
}

export function SeshWithLocationFromJSONTyped(json: any, ignoreDiscriminator: boolean): SeshWithLocation {
    if (json == null) {
        return json;
    }
    return {
        
        'createdAt': (new Date(json['created_at'])),
        'end': json['end'] == null ? undefined : (new Date(json['end'])),
        'location': LocationFromJSON(json['location']),
        'notes': json['notes'] == null ? undefined : json['notes'],
        'seshId': json['sesh_id'],
        'start': (new Date(json['start'])),
        'updatedAt': (new Date(json['updated_at'])),
        'userId': json['user_id'],
    };
}

export function SeshWithLocationToJSON(value?: SeshWithLocation | null): any {
    if (value == null) {
        return value;
    }
    return {
        
        'created_at': ((value['createdAt']).toISOString()),
        'end': value['end'] == null ? undefined : ((value['end'] as any).toISOString()),
        'location': LocationToJSON(value['location']),
        'notes': value['notes'],
        'sesh_id': value['seshId'],
        'start': ((value['start']).toISOString()),
        'updated_at': ((value['updatedAt']).toISOString()),
        'user_id': value['userId'],
    };
}

