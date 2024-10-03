/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { CreatePreference } from '../models/CreatePreference';
import type { Preference } from '../models/Preference';
import type { CancelablePromise } from '../core/CancelablePromise';
import { OpenAPI } from '../core/OpenAPI';
import { request as __request } from '../core/request';
export class ClBackendRoutesPreferencesPreferencesControllerService {
    /**
     * @param requestBody
     * @returns Preference Create a preference
     * @throws ApiError
     */
    public static createPreference(
        requestBody: CreatePreference,
    ): CancelablePromise<Array<Preference>> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/preferences',
            body: requestBody,
            mediaType: 'application/json',
            errors: {
                500: `Preference was not created`,
            },
        });
    }
    /**
     * @param preferenceId preference id
     * @returns any Get a preference successfully
     * @throws ApiError
     */
    public static getPreferenceByPreferenceId(
        preferenceId: string,
    ): CancelablePromise<any> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/preferences/{preference_id}',
            path: {
                'preference_id': preferenceId,
            },
            errors: {
                404: `Preference was not found`,
            },
        });
    }
    /**
     * @param preferenceId preference id
     * @returns void
     * @throws ApiError
     */
    public static deletePreference(
        preferenceId: string,
    ): CancelablePromise<void> {
        return __request(OpenAPI, {
            method: 'DELETE',
            url: '/preferences/{preference_id}',
            path: {
                'preference_id': preferenceId,
            },
            errors: {
                500: `Preference was not deleted`,
            },
        });
    }
    /**
     * @param userId user id
     * @returns any Get a preference(s) successfully
     * @throws ApiError
     */
    public static getPreferenceByUserId(
        userId: any,
    ): CancelablePromise<any> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/preferences/{user_id}',
            path: {
                'user_id': userId,
            },
            errors: {
                404: `No preference found`,
            },
        });
    }
}
