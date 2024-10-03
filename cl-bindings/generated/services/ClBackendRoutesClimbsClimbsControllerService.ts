/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { Climb } from '../models/Climb';
import type { CreateClimb } from '../models/CreateClimb';
import type { CancelablePromise } from '../core/CancelablePromise';
import { OpenAPI } from '../core/OpenAPI';
import { request as __request } from '../core/request';
export class ClBackendRoutesClimbsClimbsControllerService {
    /**
     * @returns any List all climbs successfully
     * @throws ApiError
     */
    public static searchClimbs(): CancelablePromise<any> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/climbs',
            errors: {
                404: `Climb was not found`,
            },
        });
    }
    /**
     * @param requestBody
     * @returns Climb Create a climb
     * @throws ApiError
     */
    public static createClimb(
        requestBody: CreateClimb,
    ): CancelablePromise<Array<Climb>> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/climbs',
            body: requestBody,
            mediaType: 'application/json',
            errors: {
                500: `Climb was not created`,
            },
        });
    }
    /**
     * @param climbId climb id
     * @returns any Get a climb successfully
     * @throws ApiError
     */
    public static getClimbByClimbId(
        climbId: string,
    ): CancelablePromise<any> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/climbs/{climb_id}',
            path: {
                'climb_id': climbId,
            },
            errors: {
                404: `Climb was not found`,
            },
        });
    }
    /**
     * @param climbId climb id
     * @returns void
     * @throws ApiError
     */
    public static deleteClimb(
        climbId: string,
    ): CancelablePromise<void> {
        return __request(OpenAPI, {
            method: 'DELETE',
            url: '/climbs/{climb_id}',
            path: {
                'climb_id': climbId,
            },
            errors: {
                500: `Climb was not deleted`,
            },
        });
    }
}
