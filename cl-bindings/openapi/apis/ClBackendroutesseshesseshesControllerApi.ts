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
  CreateSesh,
  Sesh,
  UpdateSesh,
} from '../models/index';
import {
    CreateSeshFromJSON,
    CreateSeshToJSON,
    SeshFromJSON,
    SeshToJSON,
    UpdateSeshFromJSON,
    UpdateSeshToJSON,
} from '../models/index';

export interface CreateSeshRequest {
    createSesh: CreateSesh;
}

export interface DeleteSeshRequest {
    seshId: string;
}

export interface GetSeshBySeshIdRequest {
    seshId: string;
}

export interface SearchSeshesRequest {
    notes?: string | null;
}

export interface UpdateSeshBySeshIdRequest {
    seshId: string;
    updateSesh: UpdateSesh;
}

/**
 * ClBackendroutesseshesseshesControllerApi - interface
 * 
 * @export
 * @interface ClBackendroutesseshesseshesControllerApiInterface
 */
export interface ClBackendroutesseshesseshesControllerApiInterface {
    /**
     * 
     * @param {CreateSesh} createSesh 
     * @param {*} [options] Override http request option.
     * @throws {RequiredError}
     * @memberof ClBackendroutesseshesseshesControllerApiInterface
     */
    createSeshRaw(requestParameters: CreateSeshRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<Sesh>>;

    /**
     */
    createSesh(requestParameters: CreateSeshRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<Sesh>;

    /**
     * 
     * @param {string} seshId sesh id
     * @param {*} [options] Override http request option.
     * @throws {RequiredError}
     * @memberof ClBackendroutesseshesseshesControllerApiInterface
     */
    deleteSeshRaw(requestParameters: DeleteSeshRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<void>>;

    /**
     */
    deleteSesh(requestParameters: DeleteSeshRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<void>;

    /**
     * 
     * @param {*} [options] Override http request option.
     * @throws {RequiredError}
     * @memberof ClBackendroutesseshesseshesControllerApiInterface
     */
    getActiveSeshRaw(initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<Sesh>>;

    /**
     */
    getActiveSesh(initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<Sesh>;

    /**
     * 
     * @param {string} seshId sesh id
     * @param {*} [options] Override http request option.
     * @throws {RequiredError}
     * @memberof ClBackendroutesseshesseshesControllerApiInterface
     */
    getSeshBySeshIdRaw(requestParameters: GetSeshBySeshIdRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<Sesh>>;

    /**
     */
    getSeshBySeshId(requestParameters: GetSeshBySeshIdRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<Sesh>;

    /**
     * 
     * @param {string} [notes] 
     * @param {*} [options] Override http request option.
     * @throws {RequiredError}
     * @memberof ClBackendroutesseshesseshesControllerApiInterface
     */
    searchSeshesRaw(requestParameters: SearchSeshesRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<Array<Sesh>>>;

    /**
     */
    searchSeshes(requestParameters: SearchSeshesRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<Array<Sesh>>;

    /**
     * 
     * @param {string} seshId sesh id
     * @param {UpdateSesh} updateSesh 
     * @param {*} [options] Override http request option.
     * @throws {RequiredError}
     * @memberof ClBackendroutesseshesseshesControllerApiInterface
     */
    updateSeshBySeshIdRaw(requestParameters: UpdateSeshBySeshIdRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<Sesh>>;

    /**
     */
    updateSeshBySeshId(requestParameters: UpdateSeshBySeshIdRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<Sesh>;

}

/**
 * 
 */
export class ClBackendroutesseshesseshesControllerApi extends runtime.BaseAPI implements ClBackendroutesseshesseshesControllerApiInterface {

    /**
     */
    async createSeshRaw(requestParameters: CreateSeshRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<Sesh>> {
        if (requestParameters['createSesh'] == null) {
            throw new runtime.RequiredError(
                'createSesh',
                'Required parameter "createSesh" was null or undefined when calling createSesh().'
            );
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        headerParameters['Content-Type'] = 'application/json';

        const response = await this.request({
            path: `/seshes`,
            method: 'POST',
            headers: headerParameters,
            query: queryParameters,
            body: CreateSeshToJSON(requestParameters['createSesh']),
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => SeshFromJSON(jsonValue));
    }

    /**
     */
    async createSesh(requestParameters: CreateSeshRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<Sesh> {
        const response = await this.createSeshRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     */
    async deleteSeshRaw(requestParameters: DeleteSeshRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<void>> {
        if (requestParameters['seshId'] == null) {
            throw new runtime.RequiredError(
                'seshId',
                'Required parameter "seshId" was null or undefined when calling deleteSesh().'
            );
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/seshes/{sesh_id}`.replace(`{${"sesh_id"}}`, encodeURIComponent(String(requestParameters['seshId']))),
            method: 'DELETE',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.VoidApiResponse(response);
    }

    /**
     */
    async deleteSesh(requestParameters: DeleteSeshRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<void> {
        await this.deleteSeshRaw(requestParameters, initOverrides);
    }

    /**
     */
    async getActiveSeshRaw(initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<Sesh>> {
        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/seshes/active`,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => SeshFromJSON(jsonValue));
    }

    /**
     */
    async getActiveSesh(initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<Sesh> {
        const response = await this.getActiveSeshRaw(initOverrides);
        return await response.value();
    }

    /**
     */
    async getSeshBySeshIdRaw(requestParameters: GetSeshBySeshIdRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<Sesh>> {
        if (requestParameters['seshId'] == null) {
            throw new runtime.RequiredError(
                'seshId',
                'Required parameter "seshId" was null or undefined when calling getSeshBySeshId().'
            );
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/seshes/{sesh_id}`.replace(`{${"sesh_id"}}`, encodeURIComponent(String(requestParameters['seshId']))),
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => SeshFromJSON(jsonValue));
    }

    /**
     */
    async getSeshBySeshId(requestParameters: GetSeshBySeshIdRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<Sesh> {
        const response = await this.getSeshBySeshIdRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     */
    async searchSeshesRaw(requestParameters: SearchSeshesRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<Array<Sesh>>> {
        const queryParameters: any = {};

        if (requestParameters['notes'] != null) {
            queryParameters['notes'] = requestParameters['notes'];
        }

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/seshes`,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => jsonValue.map(SeshFromJSON));
    }

    /**
     */
    async searchSeshes(requestParameters: SearchSeshesRequest = {}, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<Array<Sesh>> {
        const response = await this.searchSeshesRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     */
    async updateSeshBySeshIdRaw(requestParameters: UpdateSeshBySeshIdRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<Sesh>> {
        if (requestParameters['seshId'] == null) {
            throw new runtime.RequiredError(
                'seshId',
                'Required parameter "seshId" was null or undefined when calling updateSeshBySeshId().'
            );
        }

        if (requestParameters['updateSesh'] == null) {
            throw new runtime.RequiredError(
                'updateSesh',
                'Required parameter "updateSesh" was null or undefined when calling updateSeshBySeshId().'
            );
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        headerParameters['Content-Type'] = 'application/json';

        const response = await this.request({
            path: `/seshes/{sesh_id}`.replace(`{${"sesh_id"}}`, encodeURIComponent(String(requestParameters['seshId']))),
            method: 'PATCH',
            headers: headerParameters,
            query: queryParameters,
            body: UpdateSeshToJSON(requestParameters['updateSesh']),
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => SeshFromJSON(jsonValue));
    }

    /**
     */
    async updateSeshBySeshId(requestParameters: UpdateSeshBySeshIdRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<Sesh> {
        const response = await this.updateSeshBySeshIdRaw(requestParameters, initOverrides);
        return await response.value();
    }

}
