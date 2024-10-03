/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { CreateLocation } from '../models/CreateLocation';
import type { Location } from '../models/Location';
import type { UpdateLocation } from '../models/UpdateLocation';
import type { CancelablePromise } from '../core/CancelablePromise';
import { OpenAPI } from '../core/OpenAPI';
import { request as __request } from '../core/request';
export class ClBackendRoutesLocationsLocationsControllerService {
    /**
     * @param name
     * @returns any Get location(s) successfully
     * @throws ApiError
     */
    public static searchLocations(
        name?: string | null,
    ): CancelablePromise<any> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/locations',
            query: {
                'name': name,
            },
            errors: {
                404: `No location found`,
            },
        });
    }
    /**
     * @param requestBody
     * @returns Location Create a location
     * @throws ApiError
     */
    public static createLocation(
        requestBody: CreateLocation,
    ): CancelablePromise<Array<Location>> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/locations',
            body: requestBody,
            mediaType: 'application/json',
            errors: {
                500: `Location was not created`,
            },
        });
    }
    /**
     * @param locationId location id
     * @returns any Get a location successfully
     * @throws ApiError
     */
    public static getLocationByLocationId(
        locationId: string,
    ): CancelablePromise<any> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/locations/{location_id}',
            path: {
                'location_id': locationId,
            },
            errors: {
                404: `Location was not found`,
            },
        });
    }
    /**
     * @param locationId location id
     * @returns void
     * @throws ApiError
     */
    public static deleteLocationByLocationId(
        locationId: string,
    ): CancelablePromise<void> {
        return __request(OpenAPI, {
            method: 'DELETE',
            url: '/locations/{location_id}',
            path: {
                'location_id': locationId,
            },
            errors: {
                500: `Location was not deleted`,
            },
        });
    }
    /**
     * @param locationId location id
     * @param requestBody
     * @returns any Update location successfully
     * @throws ApiError
     */
    public static updateLocationByLocationId(
        locationId: string,
        requestBody: UpdateLocation,
    ): CancelablePromise<any> {
        return __request(OpenAPI, {
            method: 'PATCH',
            url: '/locations/{location_id}',
            path: {
                'location_id': locationId,
            },
            body: requestBody,
            mediaType: 'application/json',
            errors: {
                500: `Location was not updated`,
            },
        });
    }
}
