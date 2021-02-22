import Joi from "joi";

export const pageSchema = Joi.object<{page:number,count:number,created:number|undefined,begin_time:number|undefined,end_time:number|undefined}>({
    page:Joi.number().optional().default(1),//每页记录
    count:Joi.number().optional().default(5),//偏移量,
    // created:Joi.number().optional(),
    begin_time:Joi.number().optional(),
    end_time:Joi.number().optional(),
})
