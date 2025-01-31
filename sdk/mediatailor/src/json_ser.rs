// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_configure_logs_for_playback_configuration_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ConfigureLogsForPlaybackConfigurationInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    {
        object.key("PercentEnabled").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.percent_enabled).into()),
        );
    }
    if let Some(var_1) = &input.playback_configuration_name {
        object.key("PlaybackConfigurationName").string(var_1);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_channel_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateChannelInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_2) = &input.filler_slate {
        let mut object_3 = object.key("FillerSlate").start_object();
        crate::json_ser::serialize_structure_crate_model_slate_source(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.outputs {
        let mut array_5 = object.key("Outputs").start_array();
        for item_6 in var_4 {
            {
                let mut object_7 = array_5.value().start_object();
                crate::json_ser::serialize_structure_crate_model_request_output_item(
                    &mut object_7,
                    item_6,
                )?;
                object_7.finish();
            }
        }
        array_5.finish();
    }
    if let Some(var_8) = &input.playback_mode {
        object.key("PlaybackMode").string(var_8.as_str());
    }
    if let Some(var_9) = &input.tags {
        let mut object_10 = object.key("tags").start_object();
        for (key_11, value_12) in var_9 {
            {
                object_10.key(key_11).string(value_12);
            }
        }
        object_10.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_prefetch_schedule_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreatePrefetchScheduleInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_13) = &input.consumption {
        let mut object_14 = object.key("Consumption").start_object();
        crate::json_ser::serialize_structure_crate_model_prefetch_consumption(
            &mut object_14,
            var_13,
        )?;
        object_14.finish();
    }
    if let Some(var_15) = &input.retrieval {
        let mut object_16 = object.key("Retrieval").start_object();
        crate::json_ser::serialize_structure_crate_model_prefetch_retrieval(
            &mut object_16,
            var_15,
        )?;
        object_16.finish();
    }
    if let Some(var_17) = &input.stream_id {
        object.key("StreamId").string(var_17);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_program_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateProgramInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_18) = &input.ad_breaks {
        let mut array_19 = object.key("AdBreaks").start_array();
        for item_20 in var_18 {
            {
                let mut object_21 = array_19.value().start_object();
                crate::json_ser::serialize_structure_crate_model_ad_break(&mut object_21, item_20)?;
                object_21.finish();
            }
        }
        array_19.finish();
    }
    if let Some(var_22) = &input.schedule_configuration {
        let mut object_23 = object.key("ScheduleConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_schedule_configuration(
            &mut object_23,
            var_22,
        )?;
        object_23.finish();
    }
    if let Some(var_24) = &input.source_location_name {
        object.key("SourceLocationName").string(var_24);
    }
    if let Some(var_25) = &input.vod_source_name {
        object.key("VodSourceName").string(var_25);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_source_location_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateSourceLocationInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_26) = &input.access_configuration {
        let mut object_27 = object.key("AccessConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_access_configuration(
            &mut object_27,
            var_26,
        )?;
        object_27.finish();
    }
    if let Some(var_28) = &input.default_segment_delivery_configuration {
        let mut object_29 = object
            .key("DefaultSegmentDeliveryConfiguration")
            .start_object();
        crate::json_ser::serialize_structure_crate_model_default_segment_delivery_configuration(
            &mut object_29,
            var_28,
        )?;
        object_29.finish();
    }
    if let Some(var_30) = &input.http_configuration {
        let mut object_31 = object.key("HttpConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_http_configuration(
            &mut object_31,
            var_30,
        )?;
        object_31.finish();
    }
    if let Some(var_32) = &input.segment_delivery_configurations {
        let mut array_33 = object.key("SegmentDeliveryConfigurations").start_array();
        for item_34 in var_32 {
            {
                let mut object_35 = array_33.value().start_object();
                crate::json_ser::serialize_structure_crate_model_segment_delivery_configuration(
                    &mut object_35,
                    item_34,
                )?;
                object_35.finish();
            }
        }
        array_33.finish();
    }
    if let Some(var_36) = &input.tags {
        let mut object_37 = object.key("tags").start_object();
        for (key_38, value_39) in var_36 {
            {
                object_37.key(key_38).string(value_39);
            }
        }
        object_37.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_vod_source_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateVodSourceInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_40) = &input.http_package_configurations {
        let mut array_41 = object.key("HttpPackageConfigurations").start_array();
        for item_42 in var_40 {
            {
                let mut object_43 = array_41.value().start_object();
                crate::json_ser::serialize_structure_crate_model_http_package_configuration(
                    &mut object_43,
                    item_42,
                )?;
                object_43.finish();
            }
        }
        array_41.finish();
    }
    if let Some(var_44) = &input.tags {
        let mut object_45 = object.key("tags").start_object();
        for (key_46, value_47) in var_44 {
            {
                object_45.key(key_46).string(value_47);
            }
        }
        object_45.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_prefetch_schedules_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListPrefetchSchedulesInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if input.max_results != 0 {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.max_results).into()),
        );
    }
    if let Some(var_48) = &input.next_token {
        object.key("NextToken").string(var_48);
    }
    if let Some(var_49) = &input.stream_id {
        object.key("StreamId").string(var_49);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_put_channel_policy_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutChannelPolicyInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_50) = &input.policy {
        object.key("Policy").string(var_50);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_put_playback_configuration_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutPlaybackConfigurationInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_51) = &input.ad_decision_server_url {
        object.key("AdDecisionServerUrl").string(var_51);
    }
    if let Some(var_52) = &input.avail_suppression {
        let mut object_53 = object.key("AvailSuppression").start_object();
        crate::json_ser::serialize_structure_crate_model_avail_suppression(&mut object_53, var_52)?;
        object_53.finish();
    }
    if let Some(var_54) = &input.bumper {
        let mut object_55 = object.key("Bumper").start_object();
        crate::json_ser::serialize_structure_crate_model_bumper(&mut object_55, var_54)?;
        object_55.finish();
    }
    if let Some(var_56) = &input.cdn_configuration {
        let mut object_57 = object.key("CdnConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_cdn_configuration(&mut object_57, var_56)?;
        object_57.finish();
    }
    if let Some(var_58) = &input.configuration_aliases {
        let mut object_59 = object.key("ConfigurationAliases").start_object();
        for (key_60, value_61) in var_58 {
            {
                let mut object_62 = object_59.key(key_60).start_object();
                for (key_63, value_64) in value_61 {
                    {
                        object_62.key(key_63).string(value_64);
                    }
                }
                object_62.finish();
            }
        }
        object_59.finish();
    }
    if let Some(var_65) = &input.dash_configuration {
        let mut object_66 = object.key("DashConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_dash_configuration_for_put(
            &mut object_66,
            var_65,
        )?;
        object_66.finish();
    }
    if let Some(var_67) = &input.live_pre_roll_configuration {
        let mut object_68 = object.key("LivePreRollConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_live_pre_roll_configuration(
            &mut object_68,
            var_67,
        )?;
        object_68.finish();
    }
    if let Some(var_69) = &input.manifest_processing_rules {
        let mut object_70 = object.key("ManifestProcessingRules").start_object();
        crate::json_ser::serialize_structure_crate_model_manifest_processing_rules(
            &mut object_70,
            var_69,
        )?;
        object_70.finish();
    }
    if let Some(var_71) = &input.name {
        object.key("Name").string(var_71);
    }
    if input.personalization_threshold_seconds != 0 {
        object.key("PersonalizationThresholdSeconds").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.personalization_threshold_seconds).into()),
        );
    }
    if let Some(var_72) = &input.slate_ad_url {
        object.key("SlateAdUrl").string(var_72);
    }
    if let Some(var_73) = &input.tags {
        let mut object_74 = object.key("tags").start_object();
        for (key_75, value_76) in var_73 {
            {
                object_74.key(key_75).string(value_76);
            }
        }
        object_74.finish();
    }
    if let Some(var_77) = &input.transcode_profile_name {
        object.key("TranscodeProfileName").string(var_77);
    }
    if let Some(var_78) = &input.video_content_source_url {
        object.key("VideoContentSourceUrl").string(var_78);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_79) = &input.tags {
        let mut object_80 = object.key("tags").start_object();
        for (key_81, value_82) in var_79 {
            {
                object_80.key(key_81).string(value_82);
            }
        }
        object_80.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_channel_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateChannelInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_83) = &input.filler_slate {
        let mut object_84 = object.key("FillerSlate").start_object();
        crate::json_ser::serialize_structure_crate_model_slate_source(&mut object_84, var_83)?;
        object_84.finish();
    }
    if let Some(var_85) = &input.outputs {
        let mut array_86 = object.key("Outputs").start_array();
        for item_87 in var_85 {
            {
                let mut object_88 = array_86.value().start_object();
                crate::json_ser::serialize_structure_crate_model_request_output_item(
                    &mut object_88,
                    item_87,
                )?;
                object_88.finish();
            }
        }
        array_86.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_source_location_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateSourceLocationInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_89) = &input.access_configuration {
        let mut object_90 = object.key("AccessConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_access_configuration(
            &mut object_90,
            var_89,
        )?;
        object_90.finish();
    }
    if let Some(var_91) = &input.default_segment_delivery_configuration {
        let mut object_92 = object
            .key("DefaultSegmentDeliveryConfiguration")
            .start_object();
        crate::json_ser::serialize_structure_crate_model_default_segment_delivery_configuration(
            &mut object_92,
            var_91,
        )?;
        object_92.finish();
    }
    if let Some(var_93) = &input.http_configuration {
        let mut object_94 = object.key("HttpConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_http_configuration(
            &mut object_94,
            var_93,
        )?;
        object_94.finish();
    }
    if let Some(var_95) = &input.segment_delivery_configurations {
        let mut array_96 = object.key("SegmentDeliveryConfigurations").start_array();
        for item_97 in var_95 {
            {
                let mut object_98 = array_96.value().start_object();
                crate::json_ser::serialize_structure_crate_model_segment_delivery_configuration(
                    &mut object_98,
                    item_97,
                )?;
                object_98.finish();
            }
        }
        array_96.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_vod_source_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateVodSourceInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_99) = &input.http_package_configurations {
        let mut array_100 = object.key("HttpPackageConfigurations").start_array();
        for item_101 in var_99 {
            {
                let mut object_102 = array_100.value().start_object();
                crate::json_ser::serialize_structure_crate_model_http_package_configuration(
                    &mut object_102,
                    item_101,
                )?;
                object_102.finish();
            }
        }
        array_100.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_slate_source(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SlateSource,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_103) = &input.source_location_name {
        object.key("SourceLocationName").string(var_103);
    }
    if let Some(var_104) = &input.vod_source_name {
        object.key("VodSourceName").string(var_104);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_request_output_item(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::RequestOutputItem,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_105) = &input.dash_playlist_settings {
        let mut object_106 = object.key("DashPlaylistSettings").start_object();
        crate::json_ser::serialize_structure_crate_model_dash_playlist_settings(
            &mut object_106,
            var_105,
        )?;
        object_106.finish();
    }
    if let Some(var_107) = &input.hls_playlist_settings {
        let mut object_108 = object.key("HlsPlaylistSettings").start_object();
        crate::json_ser::serialize_structure_crate_model_hls_playlist_settings(
            &mut object_108,
            var_107,
        )?;
        object_108.finish();
    }
    if let Some(var_109) = &input.manifest_name {
        object.key("ManifestName").string(var_109);
    }
    if let Some(var_110) = &input.source_group {
        object.key("SourceGroup").string(var_110);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_prefetch_consumption(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::PrefetchConsumption,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_111) = &input.avail_matching_criteria {
        let mut array_112 = object.key("AvailMatchingCriteria").start_array();
        for item_113 in var_111 {
            {
                let mut object_114 = array_112.value().start_object();
                crate::json_ser::serialize_structure_crate_model_avail_matching_criteria(
                    &mut object_114,
                    item_113,
                )?;
                object_114.finish();
            }
        }
        array_112.finish();
    }
    if let Some(var_115) = &input.end_time {
        object
            .key("EndTime")
            .date_time(var_115, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_116) = &input.start_time {
        object
            .key("StartTime")
            .date_time(var_116, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    Ok(())
}

pub fn serialize_structure_crate_model_prefetch_retrieval(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::PrefetchRetrieval,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_117) = &input.dynamic_variables {
        let mut object_118 = object.key("DynamicVariables").start_object();
        for (key_119, value_120) in var_117 {
            {
                object_118.key(key_119).string(value_120);
            }
        }
        object_118.finish();
    }
    if let Some(var_121) = &input.end_time {
        object
            .key("EndTime")
            .date_time(var_121, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_122) = &input.start_time {
        object
            .key("StartTime")
            .date_time(var_122, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    Ok(())
}

pub fn serialize_structure_crate_model_ad_break(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::AdBreak,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_123) = &input.message_type {
        object.key("MessageType").string(var_123.as_str());
    }
    if input.offset_millis != 0 {
        object.key("OffsetMillis").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.offset_millis).into()),
        );
    }
    if let Some(var_124) = &input.slate {
        let mut object_125 = object.key("Slate").start_object();
        crate::json_ser::serialize_structure_crate_model_slate_source(&mut object_125, var_124)?;
        object_125.finish();
    }
    if let Some(var_126) = &input.splice_insert_message {
        let mut object_127 = object.key("SpliceInsertMessage").start_object();
        crate::json_ser::serialize_structure_crate_model_splice_insert_message(
            &mut object_127,
            var_126,
        )?;
        object_127.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_schedule_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ScheduleConfiguration,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_128) = &input.transition {
        let mut object_129 = object.key("Transition").start_object();
        crate::json_ser::serialize_structure_crate_model_transition(&mut object_129, var_128)?;
        object_129.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_access_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::AccessConfiguration,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_130) = &input.access_type {
        object.key("AccessType").string(var_130.as_str());
    }
    if let Some(var_131) = &input.secrets_manager_access_token_configuration {
        let mut object_132 = object
            .key("SecretsManagerAccessTokenConfiguration")
            .start_object();
        crate::json_ser::serialize_structure_crate_model_secrets_manager_access_token_configuration(&mut object_132, var_131)?;
        object_132.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_default_segment_delivery_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DefaultSegmentDeliveryConfiguration,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_133) = &input.base_url {
        object.key("BaseUrl").string(var_133);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_http_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::HttpConfiguration,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_134) = &input.base_url {
        object.key("BaseUrl").string(var_134);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_segment_delivery_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SegmentDeliveryConfiguration,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_135) = &input.base_url {
        object.key("BaseUrl").string(var_135);
    }
    if let Some(var_136) = &input.name {
        object.key("Name").string(var_136);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_http_package_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::HttpPackageConfiguration,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_137) = &input.path {
        object.key("Path").string(var_137);
    }
    if let Some(var_138) = &input.source_group {
        object.key("SourceGroup").string(var_138);
    }
    if let Some(var_139) = &input.r#type {
        object.key("Type").string(var_139.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_avail_suppression(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::AvailSuppression,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_140) = &input.mode {
        object.key("Mode").string(var_140.as_str());
    }
    if let Some(var_141) = &input.value {
        object.key("Value").string(var_141);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_bumper(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Bumper,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_142) = &input.end_url {
        object.key("EndUrl").string(var_142);
    }
    if let Some(var_143) = &input.start_url {
        object.key("StartUrl").string(var_143);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_cdn_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CdnConfiguration,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_144) = &input.ad_segment_url_prefix {
        object.key("AdSegmentUrlPrefix").string(var_144);
    }
    if let Some(var_145) = &input.content_segment_url_prefix {
        object.key("ContentSegmentUrlPrefix").string(var_145);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_dash_configuration_for_put(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DashConfigurationForPut,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_146) = &input.mpd_location {
        object.key("MpdLocation").string(var_146);
    }
    if let Some(var_147) = &input.origin_manifest_type {
        object.key("OriginManifestType").string(var_147.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_live_pre_roll_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::LivePreRollConfiguration,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_148) = &input.ad_decision_server_url {
        object.key("AdDecisionServerUrl").string(var_148);
    }
    if input.max_duration_seconds != 0 {
        object.key("MaxDurationSeconds").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.max_duration_seconds).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_model_manifest_processing_rules(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ManifestProcessingRules,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_149) = &input.ad_marker_passthrough {
        let mut object_150 = object.key("AdMarkerPassthrough").start_object();
        crate::json_ser::serialize_structure_crate_model_ad_marker_passthrough(
            &mut object_150,
            var_149,
        )?;
        object_150.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_dash_playlist_settings(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DashPlaylistSettings,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if input.manifest_window_seconds != 0 {
        object.key("ManifestWindowSeconds").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.manifest_window_seconds).into()),
        );
    }
    if input.min_buffer_time_seconds != 0 {
        object.key("MinBufferTimeSeconds").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.min_buffer_time_seconds).into()),
        );
    }
    if input.min_update_period_seconds != 0 {
        object.key("MinUpdatePeriodSeconds").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.min_update_period_seconds).into()),
        );
    }
    if input.suggested_presentation_delay_seconds != 0 {
        object.key("SuggestedPresentationDelaySeconds").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.suggested_presentation_delay_seconds).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_model_hls_playlist_settings(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::HlsPlaylistSettings,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if input.manifest_window_seconds != 0 {
        object.key("ManifestWindowSeconds").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.manifest_window_seconds).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_model_avail_matching_criteria(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::AvailMatchingCriteria,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_151) = &input.dynamic_variable {
        object.key("DynamicVariable").string(var_151);
    }
    if let Some(var_152) = &input.operator {
        object.key("Operator").string(var_152.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_splice_insert_message(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SpliceInsertMessage,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if input.avail_num != 0 {
        object.key("AvailNum").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.avail_num).into()),
        );
    }
    if input.avails_expected != 0 {
        object.key("AvailsExpected").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.avails_expected).into()),
        );
    }
    if input.splice_event_id != 0 {
        object.key("SpliceEventId").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.splice_event_id).into()),
        );
    }
    if input.unique_program_id != 0 {
        object.key("UniqueProgramId").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.unique_program_id).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_model_transition(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Transition,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_153) = &input.relative_position {
        object.key("RelativePosition").string(var_153.as_str());
    }
    if let Some(var_154) = &input.relative_program {
        object.key("RelativeProgram").string(var_154);
    }
    if input.scheduled_start_time_millis != 0 {
        object.key("ScheduledStartTimeMillis").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.scheduled_start_time_millis).into()),
        );
    }
    if let Some(var_155) = &input.r#type {
        object.key("Type").string(var_155);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_secrets_manager_access_token_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SecretsManagerAccessTokenConfiguration,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_156) = &input.header_name {
        object.key("HeaderName").string(var_156);
    }
    if let Some(var_157) = &input.secret_arn {
        object.key("SecretArn").string(var_157);
    }
    if let Some(var_158) = &input.secret_string_key {
        object.key("SecretStringKey").string(var_158);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_ad_marker_passthrough(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::AdMarkerPassthrough,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if input.enabled {
        object.key("Enabled").boolean(input.enabled);
    }
    Ok(())
}
