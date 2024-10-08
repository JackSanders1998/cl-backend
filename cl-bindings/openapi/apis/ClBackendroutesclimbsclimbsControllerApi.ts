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


import * as runtime from '../runtime';
import type {
  Climb,
  CreateClimb,
} from '../models/index';
import {
    ClimbFromJSON,
    ClimbToJSON,
    CreateClimbFromJSON,
    CreateClimbToJSON,
} from '../models/index';

export interface CreateClimbRequest {
    createClimb: CreateClimb;
}

export interface DeleteClimbRequest {
    climbId: string;
}

export interface GetClimbByClimbIdRequest {
    climbId: string;
}

/**
 * ClBackendroutesclimbsclimbsControllerApi - interface
 * 
 * @export
 * @interface ClBackendroutesclimbsclimbsControllerApiInterface
 */
export interface ClBackendroutesclimbsclimbsControllerApiInterface {
    /**
     * 
     * @param {CreateClimb} createClimb 
     * @param {*} [options] Override http request option.
     * @throws {RequiredError}
     * @memberof ClBackendroutesclimbsclimbsControllerApiInterface
     */
    createClimbRaw(requestParameters: CreateClimbRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<Climb>>;

    /**
     */
    createClimb(requestParameters: CreateClimbRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<Climb>;

    /**
     * 
     * @param {string} climbId climb id
     * @param {*} [options] Override http request option.
     * @throws {RequiredError}
     * @memberof ClBackendroutesclimbsclimbsControllerApiInterface
     */
    deleteClimbRaw(requestParameters: DeleteClimbRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<void>>;

    /**
     */
    deleteClimb(requestParameters: DeleteClimbRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<void>;

    /**
     * 
     * @param {string} climbId climb id
     * @param {*} [options] Override http request option.
     * @throws {RequiredError}
     * @memberof ClBackendroutesclimbsclimbsControllerApiInterface
     */
    getClimbByClimbIdRaw(requestParameters: GetClimbByClimbIdRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<Climb>>;

    /**
     */
    getClimbByClimbId(requestParameters: GetClimbByClimbIdRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<Climb>;

    /**
     * 
     * @param {*} [options] Override http request option.
     * @throws {RequiredError}
     * @memberof ClBackendroutesclimbsclimbsControllerApiInterface
     */
    searchClimbsRaw(initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<Array<Climb>>>;

    /**
     */
    searchClimbs(initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<Array<Climb>>;

}

/**
 * 
 */
export class ClBackendroutesclimbsclimbsControllerApi extends runtime.BaseAPI implements ClBackendroutesclimbsclimbsControllerApiInterface {

    /**
     */
    async createClimbRaw(requestParameters: CreateClimbRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<Climb>> {
        if (requestParameters['createClimb'] == null) {
            throw new runtime.RequiredError(
                'createClimb',
                'Required parameter "createClimb" was null or undefined when calling createClimb().'
            );
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        headerParameters['Content-Type'] = 'application/json';

        const response = await this.request({
            path: `/climbs`,
            method: 'POST',
            headers: headerParameters,
            query: queryParameters,
            body: CreateClimbToJSON(requestParameters['createClimb']),
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => ClimbFromJSON(jsonValue));
    }

    /**
     */
    async createClimb(requestParameters: CreateClimbRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<Climb> {
        const response = await this.createClimbRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     */
    async deleteClimbRaw(requestParameters: DeleteClimbRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<void>> {
        if (requestParameters['climbId'] == null) {
            throw new runtime.RequiredError(
                'climbId',
                'Required parameter "climbId" was null or undefined when calling deleteClimb().'
            );
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/climbs/{climb_id}`.replace(`{${"climb_id"}}`, encodeURIComponent(String(requestParameters['climbId']))),
            method: 'DELETE',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.VoidApiResponse(response);
    }

    /**
     */
    async deleteClimb(requestParameters: DeleteClimbRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<void> {
        await this.deleteClimbRaw(requestParameters, initOverrides);
    }

    /**
     */
    async getClimbByClimbIdRaw(requestParameters: GetClimbByClimbIdRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<Climb>> {
        if (requestParameters['climbId'] == null) {
            throw new runtime.RequiredError(
                'climbId',
                'Required parameter "climbId" was null or undefined when calling getClimbByClimbId().'
            );
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/climbs/{climb_id}`.replace(`{${"climb_id"}}`, encodeURIComponent(String(requestParameters['climbId']))),
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => ClimbFromJSON(jsonValue));
    }

    /**
     */
    async getClimbByClimbId(requestParameters: GetClimbByClimbIdRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<Climb> {
        const response = await this.getClimbByClimbIdRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     */
    async searchClimbsRaw(initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<Array<Climb>>> {
        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/climbs`,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => jsonValue.map(ClimbFromJSON));
    }

    /**
     */
    async searchClimbs(initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<Array<Climb>> {
        const response = await this.searchClimbsRaw(initOverrides);
        return await response.value();
    }

}
