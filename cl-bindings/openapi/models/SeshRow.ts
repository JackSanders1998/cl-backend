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
 * @interface SeshRow
 */
export interface SeshRow {
    /**
     * 
     * @type {Date}
     * @memberof SeshRow
     */
    createdAt: Date;
    /**
     * 
     * @type {Date}
     * @memberof SeshRow
     */
    end?: Date | null;
    /**
     * 
     * @type {string}
     * @memberof SeshRow
     */
    locationId: string;
    /**
     * 
     * @type {string}
     * @memberof SeshRow
     */
    notes?: string | null;
    /**
     * 
     * @type {string}
     * @memberof SeshRow
     */
    seshId: string;
    /**
     * 
     * @type {Date}
     * @memberof SeshRow
     */
    start: Date;
    /**
     * 
     * @type {Date}
     * @memberof SeshRow
     */
    updatedAt: Date;
    /**
     * 
     * @type {string}
     * @memberof SeshRow
     */
    userId: string;
}

/**
 * Check if a given object implements the SeshRow interface.
 */
export function instanceOfSeshRow(value: object): value is SeshRow {
    if (!('createdAt' in value) || value['createdAt'] === undefined) return false;
    if (!('locationId' in value) || value['locationId'] === undefined) return false;
    if (!('seshId' in value) || value['seshId'] === undefined) return false;
    if (!('start' in value) || value['start'] === undefined) return false;
    if (!('updatedAt' in value) || value['updatedAt'] === undefined) return false;
    if (!('userId' in value) || value['userId'] === undefined) return false;
    return true;
}

export function SeshRowFromJSON(json: any): SeshRow {
    return SeshRowFromJSONTyped(json, false);
}

export function SeshRowFromJSONTyped(json: any, ignoreDiscriminator: boolean): SeshRow {
    if (json == null) {
        return json;
    }
    return {
        
        'createdAt': (new Date(json['created_at'])),
        'end': json['end'] == null ? undefined : (new Date(json['end'])),
        'locationId': json['location_id'],
        'notes': json['notes'] == null ? undefined : json['notes'],
        'seshId': json['sesh_id'],
        'start': (new Date(json['start'])),
        'updatedAt': (new Date(json['updated_at'])),
        'userId': json['user_id'],
    };
}

export function SeshRowToJSON(value?: SeshRow | null): any {
    if (value == null) {
        return value;
    }
    return {
        
        'created_at': ((value['createdAt']).toISOString()),
        'end': value['end'] == null ? undefined : ((value['end'] as any).toISOString()),
        'location_id': value['locationId'],
        'notes': value['notes'],
        'sesh_id': value['seshId'],
        'start': ((value['start']).toISOString()),
        'updated_at': ((value['updatedAt']).toISOString()),
        'user_id': value['userId'],
    };
}

