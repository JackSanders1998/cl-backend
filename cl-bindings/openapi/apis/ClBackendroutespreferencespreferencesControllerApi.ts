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
  CreatePreference,
  Preference,
} from '../models/index';
import {
    CreatePreferenceFromJSON,
    CreatePreferenceToJSON,
    PreferenceFromJSON,
    PreferenceToJSON,
} from '../models/index';

export interface CreatePreferenceRequest {
    createPreference: CreatePreference;
}

export interface DeletePreferenceRequest {
    preferenceId: string;
}

export interface GetPreferenceByPreferenceIdRequest {
    preferenceId: string;
}

/**
 * ClBackendroutespreferencespreferencesControllerApi - interface
 * 
 * @export
 * @interface ClBackendroutespreferencespreferencesControllerApiInterface
 */
export interface ClBackendroutespreferencespreferencesControllerApiInterface {
    /**
     * 
     * @param {CreatePreference} createPreference 
     * @param {*} [options] Override http request option.
     * @throws {RequiredError}
     * @memberof ClBackendroutespreferencespreferencesControllerApiInterface
     */
    createPreferenceRaw(requestParameters: CreatePreferenceRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<Preference>>;

    /**
     */
    createPreference(requestParameters: CreatePreferenceRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<Preference>;

    /**
     * 
     * @param {string} preferenceId preference id
     * @param {*} [options] Override http request option.
     * @throws {RequiredError}
     * @memberof ClBackendroutespreferencespreferencesControllerApiInterface
     */
    deletePreferenceRaw(requestParameters: DeletePreferenceRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<void>>;

    /**
     */
    deletePreference(requestParameters: DeletePreferenceRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<void>;

    /**
     * 
     * @param {string} preferenceId preference id
     * @param {*} [options] Override http request option.
     * @throws {RequiredError}
     * @memberof ClBackendroutespreferencespreferencesControllerApiInterface
     */
    getPreferenceByPreferenceIdRaw(requestParameters: GetPreferenceByPreferenceIdRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<Preference>>;

    /**
     */
    getPreferenceByPreferenceId(requestParameters: GetPreferenceByPreferenceIdRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<Preference>;

    /**
     * 
     * @param {*} [options] Override http request option.
     * @throws {RequiredError}
     * @memberof ClBackendroutespreferencespreferencesControllerApiInterface
     */
    getPreferenceByUserIdRaw(initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<Array<Preference>>>;

    /**
     */
    getPreferenceByUserId(initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<Array<Preference>>;

}

/**
 * 
 */
export class ClBackendroutespreferencespreferencesControllerApi extends runtime.BaseAPI implements ClBackendroutespreferencespreferencesControllerApiInterface {

    /**
     */
    async createPreferenceRaw(requestParameters: CreatePreferenceRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<Preference>> {
        if (requestParameters['createPreference'] == null) {
            throw new runtime.RequiredError(
                'createPreference',
                'Required parameter "createPreference" was null or undefined when calling createPreference().'
            );
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        headerParameters['Content-Type'] = 'application/json';

        const response = await this.request({
            path: `/preferences`,
            method: 'POST',
            headers: headerParameters,
            query: queryParameters,
            body: CreatePreferenceToJSON(requestParameters['createPreference']),
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => PreferenceFromJSON(jsonValue));
    }

    /**
     */
    async createPreference(requestParameters: CreatePreferenceRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<Preference> {
        const response = await this.createPreferenceRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     */
    async deletePreferenceRaw(requestParameters: DeletePreferenceRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<void>> {
        if (requestParameters['preferenceId'] == null) {
            throw new runtime.RequiredError(
                'preferenceId',
                'Required parameter "preferenceId" was null or undefined when calling deletePreference().'
            );
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/preferences/{preference_id}`.replace(`{${"preference_id"}}`, encodeURIComponent(String(requestParameters['preferenceId']))),
            method: 'DELETE',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.VoidApiResponse(response);
    }

    /**
     */
    async deletePreference(requestParameters: DeletePreferenceRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<void> {
        await this.deletePreferenceRaw(requestParameters, initOverrides);
    }

    /**
     */
    async getPreferenceByPreferenceIdRaw(requestParameters: GetPreferenceByPreferenceIdRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<Preference>> {
        if (requestParameters['preferenceId'] == null) {
            throw new runtime.RequiredError(
                'preferenceId',
                'Required parameter "preferenceId" was null or undefined when calling getPreferenceByPreferenceId().'
            );
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/preferences/{preference_id}`.replace(`{${"preference_id"}}`, encodeURIComponent(String(requestParameters['preferenceId']))),
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => PreferenceFromJSON(jsonValue));
    }

    /**
     */
    async getPreferenceByPreferenceId(requestParameters: GetPreferenceByPreferenceIdRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<Preference> {
        const response = await this.getPreferenceByPreferenceIdRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     */
    async getPreferenceByUserIdRaw(initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<Array<Preference>>> {
        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/preferences`,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => jsonValue.map(PreferenceFromJSON));
    }

    /**
     */
    async getPreferenceByUserId(initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<Array<Preference>> {
        const response = await this.getPreferenceByUserIdRaw(initOverrides);
        return await response.value();
    }

}
