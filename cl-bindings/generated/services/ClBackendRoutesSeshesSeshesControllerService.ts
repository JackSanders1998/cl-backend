/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { CreateSesh } from '../models/CreateSesh';
import type { Sesh } from '../models/Sesh';
import type { UpdateSesh } from '../models/UpdateSesh';
import type { CancelablePromise } from '../core/CancelablePromise';
import { OpenAPI } from '../core/OpenAPI';
import { request as __request } from '../core/request';
export class ClBackendRoutesSeshesSeshesControllerService {
    /**
     * @param notes
     * @returns any Get sesh(es) successfully
     * @throws ApiError
     */
    public static searchSeshes(
        notes?: string | null,
    ): CancelablePromise<any> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/seshes',
            query: {
                'notes': notes,
            },
            errors: {
                404: `No sesh found`,
            },
        });
    }
    /**
     * @param requestBody
     * @returns Sesh Create a sesh
     * @throws ApiError
     */
    public static createSesh(
        requestBody: CreateSesh,
    ): CancelablePromise<Array<Sesh>> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/seshes',
            body: requestBody,
            mediaType: 'application/json',
            errors: {
                500: `Sesh was not created`,
            },
        });
    }
    /**
     * @returns any Get active sesh successfully
     * @throws ApiError
     */
    public static getActiveSesh(): CancelablePromise<any> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/seshes/active',
            errors: {
                404: `No active sesh found`,
            },
        });
    }
    /**
     * @param seshId sesh id
     * @returns any Get a sesh successfully
     * @throws ApiError
     */
    public static getSeshBySeshId(
        seshId: string,
    ): CancelablePromise<any> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/seshes/{sesh_id}',
            path: {
                'sesh_id': seshId,
            },
            errors: {
                404: `Sesh was not found`,
            },
        });
    }
    /**
     * @param seshId sesh id
     * @returns void
     * @throws ApiError
     */
    public static deleteSesh(
        seshId: string,
    ): CancelablePromise<void> {
        return __request(OpenAPI, {
            method: 'DELETE',
            url: '/seshes/{sesh_id}',
            path: {
                'sesh_id': seshId,
            },
            errors: {
                500: `Sesh was not deleted`,
            },
        });
    }
    /**
     * @param seshId sesh id
     * @param requestBody
     * @returns any Update sesh successfully
     * @throws ApiError
     */
    public static updateSeshBySeshId(
        seshId: string,
        requestBody: UpdateSesh,
    ): CancelablePromise<any> {
        return __request(OpenAPI, {
            method: 'PATCH',
            url: '/seshes/{sesh_id}',
            path: {
                'sesh_id': seshId,
            },
            body: requestBody,
            mediaType: 'application/json',
            errors: {
                500: `Sesh was not updated`,
            },
        });
    }
}
