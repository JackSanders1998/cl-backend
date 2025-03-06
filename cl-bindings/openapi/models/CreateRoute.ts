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
import type { Scale } from './Scale';
import {
    ScaleFromJSON,
    ScaleFromJSONTyped,
    ScaleToJSON,
} from './Scale';
import type { Discipline } from './Discipline';
import {
    DisciplineFromJSON,
    DisciplineFromJSONTyped,
    DisciplineToJSON,
} from './Discipline';

/**
 * 
 * @export
 * @interface CreateRoute
 */
export interface CreateRoute {
    /**
     * 
     * @type {string}
     * @memberof CreateRoute
     */
    author: string;
    /**
     * 
     * @type {string}
     * @memberof CreateRoute
     */
    description?: string | null;
    /**
     * 
     * @type {Array<Discipline>}
     * @memberof CreateRoute
     */
    disciplines: Array<Discipline>;
    /**
     * 
     * @type {string}
     * @memberof CreateRoute
     */
    grade: string;
    /**
     * 
     * @type {string}
     * @memberof CreateRoute
     */
    locationId: string;
    /**
     * 
     * @type {Scale}
     * @memberof CreateRoute
     */
    scale: Scale;
}



/**
 * Check if a given object implements the CreateRoute interface.
 */
export function instanceOfCreateRoute(value: object): value is CreateRoute {
    if (!('author' in value) || value['author'] === undefined) return false;
    if (!('disciplines' in value) || value['disciplines'] === undefined) return false;
    if (!('grade' in value) || value['grade'] === undefined) return false;
    if (!('locationId' in value) || value['locationId'] === undefined) return false;
    if (!('scale' in value) || value['scale'] === undefined) return false;
    return true;
}

export function CreateRouteFromJSON(json: any): CreateRoute {
    return CreateRouteFromJSONTyped(json, false);
}

export function CreateRouteFromJSONTyped(json: any, ignoreDiscriminator: boolean): CreateRoute {
    if (json == null) {
        return json;
    }
    return {
        
        'author': json['author'],
        'description': json['description'] == null ? undefined : json['description'],
        'disciplines': ((json['disciplines'] as Array<any>).map(DisciplineFromJSON)),
        'grade': json['grade'],
        'locationId': json['location_id'],
        'scale': ScaleFromJSON(json['scale']),
    };
}

export function CreateRouteToJSON(value?: CreateRoute | null): any {
    if (value == null) {
        return value;
    }
    return {
        
        'author': value['author'],
        'description': value['description'],
        'disciplines': ((value['disciplines'] as Array<any>).map(DisciplineToJSON)),
        'grade': value['grade'],
        'location_id': value['locationId'],
        'scale': ScaleToJSON(value['scale']),
    };
}

