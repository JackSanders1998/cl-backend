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
 * @interface SqlxSeshWithLocationAndClimbs
 */
export interface SqlxSeshWithLocationAndClimbs {
    /**
     * 
     * @type {Attempt}
     * @memberof SqlxSeshWithLocationAndClimbs
     */
    attempt?: Attempt | null;
    /**
     * 
     * @type {Date}
     * @memberof SqlxSeshWithLocationAndClimbs
     */
    climbCreatedAt?: Date | null;
    /**
     * 
     * @type {string}
     * @memberof SqlxSeshWithLocationAndClimbs
     */
    climbId?: string | null;
    /**
     * 
     * @type {string}
     * @memberof SqlxSeshWithLocationAndClimbs
     */
    climbNotes?: string | null;
    /**
     * 
     * @type {ClimbType}
     * @memberof SqlxSeshWithLocationAndClimbs
     */
    climbType?: ClimbType | null;
    /**
     * 
     * @type {Date}
     * @memberof SqlxSeshWithLocationAndClimbs
     */
    climbUpdatedAt?: Date | null;
    /**
     * 
     * @type {Date}
     * @memberof SqlxSeshWithLocationAndClimbs
     */
    createdAt: Date;
    /**
     * 
     * @type {Date}
     * @memberof SqlxSeshWithLocationAndClimbs
     */
    end?: Date | null;
    /**
     * 
     * @type {string}
     * @memberof SqlxSeshWithLocationAndClimbs
     */
    environment: string;
    /**
     * 
     * @type {string}
     * @memberof SqlxSeshWithLocationAndClimbs
     */
    grade?: string | null;
    /**
     * 
     * @type {Date}
     * @memberof SqlxSeshWithLocationAndClimbs
     */
    locationCreatedAt: Date;
    /**
     * 
     * @type {string}
     * @memberof SqlxSeshWithLocationAndClimbs
     */
    locationId: string;
    /**
     * 
     * @type {Date}
     * @memberof SqlxSeshWithLocationAndClimbs
     */
    locationUpdatedAt: Date;
    /**
     * 
     * @type {string}
     * @memberof SqlxSeshWithLocationAndClimbs
     */
    name: string;
    /**
     * 
     * @type {string}
     * @memberof SqlxSeshWithLocationAndClimbs
     */
    notes?: string | null;
    /**
     * 
     * @type {string}
     * @memberof SqlxSeshWithLocationAndClimbs
     */
    pointer?: string | null;
    /**
     * 
     * @type {Scale}
     * @memberof SqlxSeshWithLocationAndClimbs
     */
    scale?: Scale | null;
    /**
     * 
     * @type {string}
     * @memberof SqlxSeshWithLocationAndClimbs
     */
    seshId: string;
    /**
     * 
     * @type {Date}
     * @memberof SqlxSeshWithLocationAndClimbs
     */
    start: Date;
    /**
     * 
     * @type {Style}
     * @memberof SqlxSeshWithLocationAndClimbs
     */
    style?: Style | null;
    /**
     * 
     * @type {Date}
     * @memberof SqlxSeshWithLocationAndClimbs
     */
    updatedAt: Date;
    /**
     * 
     * @type {string}
     * @memberof SqlxSeshWithLocationAndClimbs
     */
    userId: string;
}



/**
 * Check if a given object implements the SqlxSeshWithLocationAndClimbs interface.
 */
export function instanceOfSqlxSeshWithLocationAndClimbs(value: object): value is SqlxSeshWithLocationAndClimbs {
    if (!('createdAt' in value) || value['createdAt'] === undefined) return false;
    if (!('environment' in value) || value['environment'] === undefined) return false;
    if (!('locationCreatedAt' in value) || value['locationCreatedAt'] === undefined) return false;
    if (!('locationId' in value) || value['locationId'] === undefined) return false;
    if (!('locationUpdatedAt' in value) || value['locationUpdatedAt'] === undefined) return false;
    if (!('name' in value) || value['name'] === undefined) return false;
    if (!('seshId' in value) || value['seshId'] === undefined) return false;
    if (!('start' in value) || value['start'] === undefined) return false;
    if (!('updatedAt' in value) || value['updatedAt'] === undefined) return false;
    if (!('userId' in value) || value['userId'] === undefined) return false;
    return true;
}

export function SqlxSeshWithLocationAndClimbsFromJSON(json: any): SqlxSeshWithLocationAndClimbs {
    return SqlxSeshWithLocationAndClimbsFromJSONTyped(json, false);
}

export function SqlxSeshWithLocationAndClimbsFromJSONTyped(json: any, ignoreDiscriminator: boolean): SqlxSeshWithLocationAndClimbs {
    if (json == null) {
        return json;
    }
    return {
        
        'attempt': json['attempt'] == null ? undefined : AttemptFromJSON(json['attempt']),
        'climbCreatedAt': json['climb_created_at'] == null ? undefined : (new Date(json['climb_created_at'])),
        'climbId': json['climb_id'] == null ? undefined : json['climb_id'],
        'climbNotes': json['climb_notes'] == null ? undefined : json['climb_notes'],
        'climbType': json['climb_type'] == null ? undefined : ClimbTypeFromJSON(json['climb_type']),
        'climbUpdatedAt': json['climb_updated_at'] == null ? undefined : (new Date(json['climb_updated_at'])),
        'createdAt': (new Date(json['created_at'])),
        'end': json['end'] == null ? undefined : (new Date(json['end'])),
        'environment': json['environment'],
        'grade': json['grade'] == null ? undefined : json['grade'],
        'locationCreatedAt': (new Date(json['location_created_at'])),
        'locationId': json['location_id'],
        'locationUpdatedAt': (new Date(json['location_updated_at'])),
        'name': json['name'],
        'notes': json['notes'] == null ? undefined : json['notes'],
        'pointer': json['pointer'] == null ? undefined : json['pointer'],
        'scale': json['scale'] == null ? undefined : ScaleFromJSON(json['scale']),
        'seshId': json['sesh_id'],
        'start': (new Date(json['start'])),
        'style': json['style'] == null ? undefined : StyleFromJSON(json['style']),
        'updatedAt': (new Date(json['updated_at'])),
        'userId': json['user_id'],
    };
}

export function SqlxSeshWithLocationAndClimbsToJSON(value?: SqlxSeshWithLocationAndClimbs | null): any {
    if (value == null) {
        return value;
    }
    return {
        
        'attempt': AttemptToJSON(value['attempt']),
        'climb_created_at': value['climbCreatedAt'] == null ? undefined : ((value['climbCreatedAt'] as any).toISOString()),
        'climb_id': value['climbId'],
        'climb_notes': value['climbNotes'],
        'climb_type': ClimbTypeToJSON(value['climbType']),
        'climb_updated_at': value['climbUpdatedAt'] == null ? undefined : ((value['climbUpdatedAt'] as any).toISOString()),
        'created_at': ((value['createdAt']).toISOString()),
        'end': value['end'] == null ? undefined : ((value['end'] as any).toISOString()),
        'environment': value['environment'],
        'grade': value['grade'],
        'location_created_at': ((value['locationCreatedAt']).toISOString()),
        'location_id': value['locationId'],
        'location_updated_at': ((value['locationUpdatedAt']).toISOString()),
        'name': value['name'],
        'notes': value['notes'],
        'pointer': value['pointer'],
        'scale': ScaleToJSON(value['scale']),
        'sesh_id': value['seshId'],
        'start': ((value['start']).toISOString()),
        'style': StyleToJSON(value['style']),
        'updated_at': ((value['updatedAt']).toISOString()),
        'user_id': value['userId'],
    };
}

