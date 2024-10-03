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
 * @interface UpdateClimb
 */
export interface UpdateClimb {
    /**
     * 
     * @type {Attempt}
     * @memberof UpdateClimb
     */
    attempt?: Attempt | null;
    /**
     * 
     * @type {ClimbType}
     * @memberof UpdateClimb
     */
    climbType?: ClimbType | null;
    /**
     * 
     * @type {string}
     * @memberof UpdateClimb
     */
    grade?: string | null;
    /**
     * 
     * @type {string}
     * @memberof UpdateClimb
     */
    notes?: string | null;
    /**
     * 
     * @type {string}
     * @memberof UpdateClimb
     */
    pointer?: string | null;
    /**
     * 
     * @type {Scale}
     * @memberof UpdateClimb
     */
    scale?: Scale | null;
    /**
     * 
     * @type {string}
     * @memberof UpdateClimb
     */
    seshId?: string | null;
    /**
     * 
     * @type {Style}
     * @memberof UpdateClimb
     */
    style?: Style | null;
}



/**
 * Check if a given object implements the UpdateClimb interface.
 */
export function instanceOfUpdateClimb(value: object): value is UpdateClimb {
    return true;
}

export function UpdateClimbFromJSON(json: any): UpdateClimb {
    return UpdateClimbFromJSONTyped(json, false);
}

export function UpdateClimbFromJSONTyped(json: any, ignoreDiscriminator: boolean): UpdateClimb {
    if (json == null) {
        return json;
    }
    return {
        
        'attempt': json['attempt'] == null ? undefined : AttemptFromJSON(json['attempt']),
        'climbType': json['climb_type'] == null ? undefined : ClimbTypeFromJSON(json['climb_type']),
        'grade': json['grade'] == null ? undefined : json['grade'],
        'notes': json['notes'] == null ? undefined : json['notes'],
        'pointer': json['pointer'] == null ? undefined : json['pointer'],
        'scale': json['scale'] == null ? undefined : ScaleFromJSON(json['scale']),
        'seshId': json['sesh_id'] == null ? undefined : json['sesh_id'],
        'style': json['style'] == null ? undefined : StyleFromJSON(json['style']),
    };
}

export function UpdateClimbToJSON(value?: UpdateClimb | null): any {
    if (value == null) {
        return value;
    }
    return {
        
        'attempt': AttemptToJSON(value['attempt']),
        'climb_type': ClimbTypeToJSON(value['climbType']),
        'grade': value['grade'],
        'notes': value['notes'],
        'pointer': value['pointer'],
        'scale': ScaleToJSON(value['scale']),
        'sesh_id': value['seshId'],
        'style': StyleToJSON(value['style']),
    };
}
