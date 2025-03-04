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
import type { Attempt } from './Attempt';
import {
    AttemptFromJSON,
    AttemptFromJSONTyped,
    AttemptToJSON,
} from './Attempt';
import type { Style } from './Style';
import {
    StyleFromJSON,
    StyleFromJSONTyped,
    StyleToJSON,
} from './Style';
import type { Scale } from './Scale';
import {
    ScaleFromJSON,
    ScaleFromJSONTyped,
    ScaleToJSON,
} from './Scale';
import type { ClimbType } from './ClimbType';
import {
    ClimbTypeFromJSON,
    ClimbTypeFromJSONTyped,
    ClimbTypeToJSON,
} from './ClimbType';

/**
 * 
 * @export
 * @interface ClimbData
 */
export interface ClimbData {
    /**
     * 
     * @type {Attempt}
     * @memberof ClimbData
     */
    attempt: Attempt;
    /**
     * 
     * @type {string}
     * @memberof ClimbData
     */
    climbId: string;
    /**
     * 
     * @type {ClimbType}
     * @memberof ClimbData
     */
    climbType: ClimbType;
    /**
     * 
     * @type {Date}
     * @memberof ClimbData
     */
    createdAt: Date;
    /**
     * 
     * @type {string}
     * @memberof ClimbData
     */
    grade: string;
    /**
     * 
     * @type {string}
     * @memberof ClimbData
     */
    notes?: string | null;
    /**
     * 
     * @type {string}
     * @memberof ClimbData
     */
    pointer?: string | null;
    /**
     * 
     * @type {Scale}
     * @memberof ClimbData
     */
    scale: Scale;
    /**
     * 
     * @type {Style}
     * @memberof ClimbData
     */
    style?: Style | null;
    /**
     * 
     * @type {Date}
     * @memberof ClimbData
     */
    updatedAt: Date;
}



/**
 * Check if a given object implements the ClimbData interface.
 */
export function instanceOfClimbData(value: object): value is ClimbData {
    if (!('attempt' in value) || value['attempt'] === undefined) return false;
    if (!('climbId' in value) || value['climbId'] === undefined) return false;
    if (!('climbType' in value) || value['climbType'] === undefined) return false;
    if (!('createdAt' in value) || value['createdAt'] === undefined) return false;
    if (!('grade' in value) || value['grade'] === undefined) return false;
    if (!('scale' in value) || value['scale'] === undefined) return false;
    if (!('updatedAt' in value) || value['updatedAt'] === undefined) return false;
    return true;
}

export function ClimbDataFromJSON(json: any): ClimbData {
    return ClimbDataFromJSONTyped(json, false);
}

export function ClimbDataFromJSONTyped(json: any, ignoreDiscriminator: boolean): ClimbData {
    if (json == null) {
        return json;
    }
    return {
        
        'attempt': AttemptFromJSON(json['attempt']),
        'climbId': json['climb_id'],
        'climbType': ClimbTypeFromJSON(json['climb_type']),
        'createdAt': (new Date(json['created_at'])),
        'grade': json['grade'],
        'notes': json['notes'] == null ? undefined : json['notes'],
        'pointer': json['pointer'] == null ? undefined : json['pointer'],
        'scale': ScaleFromJSON(json['scale']),
        'style': json['style'] == null ? undefined : StyleFromJSON(json['style']),
        'updatedAt': (new Date(json['updated_at'])),
    };
}

export function ClimbDataToJSON(value?: ClimbData | null): any {
    if (value == null) {
        return value;
    }
    return {
        
        'attempt': AttemptToJSON(value['attempt']),
        'climb_id': value['climbId'],
        'climb_type': ClimbTypeToJSON(value['climbType']),
        'created_at': ((value['createdAt']).toISOString()),
        'grade': value['grade'],
        'notes': value['notes'],
        'pointer': value['pointer'],
        'scale': ScaleToJSON(value['scale']),
        'style': StyleToJSON(value['style']),
        'updated_at': ((value['updatedAt']).toISOString()),
    };
}

