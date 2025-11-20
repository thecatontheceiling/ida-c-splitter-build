use crate::type_parser::parse_type;

// Test cases from user examples
#[test]
fn test_type_001_template_struct_with_base() {
    let result = parse_type("struct __cppobj TAdfStructPtr<SDeformPoints> : CAdfStructPtrBase");
    assert_eq!(result, vec!["TAdfStructPtr<SDeformPoints>"]);
}

#[test]
fn test_type_002_nested_namespace_struct() {
    let result = parse_type("struct __cppobj dynamo::vm::machine");
    assert_eq!(result, vec!["dynamo", "vm", "machine"]);
}

#[test]
fn test_type_003_complex_typedef() {
    let result = parse_type("typedef std::_Tree_val<std::_Tmap_traits<std::string,CUpdateQueue::MeanTracker *,std::less<std::string >,std::allocator<std::pair<std::string const ,CUpdateQueue::MeanTracker *> >,0> >::_Redbl std::_Tree_val<std::_Tmap_traits<ava::idstring<NAnimationSystem::SAnimationIdTag,0>,int,std::less<ava::idstring<NAnimationSystem::SAnimationIdTag,0> >,std::allocator<std::pair<ava::idstring<NAnimationSystem::SAnimationIdTag,0> const ,int> >,0> >::_Redbl;");
    assert_eq!(
        result,
        vec![
            "std",
            "_Tree_val<std::_Tmap_traits<ava::idstring<NAnimationSystem::SAnimationIdTag,0>,int,std::less<ava::idstring<NAnimationSystem::SAnimationIdTag,0> >,std::allocator<std::pair<ava::idstring<NAnimationSystem::SAnimationIdTag,0> const ,int> >,0> >",
            "_Redbl"
        ]
    );
}

#[test]
fn test_type_004_enum_with_underlying_type() {
    let result = parse_type("enum Scaleform::Render::PrimitiveFillFlags : __int32");
    assert_eq!(result, vec!["Scaleform", "Render", "PrimitiveFillFlags"]);
}

// Additional test cases from types_sample.txt

#[test]
fn test_type_005_simple_typedef() {
    let result = parse_type("typedef union $D91DE99308EDB4B15B954CC4B13FA8FA _Dconst;");
    assert_eq!(result, vec!["_Dconst"]);
}

#[test]
fn test_type_006_simple_enum() {
    let result = parse_type("enum SProfileOffer::OfferType : __int32");
    assert_eq!(result, vec!["SProfileOffer", "OfferType"]);
}

#[test]
fn test_type_007_enum_nested_namespace() {
    let result = parse_type("enum NVehicle::EDoorSlot : __int32");
    assert_eq!(result, vec!["NVehicle", "EDoorSlot"]);
}

#[test]
fn test_type_008_typedef_with_unnamed_tag() {
    let result = parse_type("typedef std::tr1::_Callable_base<<lambda5>,0>::<unnamed_tag> std::tr1::_Callable_base<<lambda16>,0>::<unnamed_tag>;");
    assert_eq!(
        result,
        vec!["std", "tr1", "_Callable_base<<lambda16>,0>", "<unnamed_tag>"]
    );
}

#[test]
fn test_type_009_typedef_with_template() {
    let result = parse_type("typedef hkArrayBase<boost::shared_ptr<CAnimationInstance> >::<unnamed_tag> hkArrayBase<hkResource *>::<unnamed_tag>;");
    assert_eq!(result, vec!["hkArrayBase<hkResource *>", "<unnamed_tag>"]);
}

#[test]
fn test_type_010_enum_nested_lambda() {
    let result = parse_type("enum NGSONodes::IsUpright::__l2::<unnamed_tag> : __int32");
    assert_eq!(result, vec!["NGSONodes", "IsUpright", "__l2", "<unnamed_tag>"]);
}

#[test]
fn test_type_011_enum_with_fields() {
    let result = parse_type("enum NGSONodes::TriggerWingsuit::__l2::EFields : __int32");
    assert_eq!(result, vec!["NGSONodes", "TriggerWingsuit", "__l2", "EFields"]);
}

#[test]
fn test_type_012_typedef_enum_to_enum() {
    let result = parse_type("typedef NVehicle::EDoorSlot CSniperAimer::EState;");
    assert_eq!(result, vec!["CSniperAimer", "EState"]);
}

#[test]
fn test_type_013_simple_enum_no_namespace() {
    let result = parse_type("enum EMemoryRange : __int32");
    assert_eq!(result, vec!["EMemoryRange"]);
}

#[test]
fn test_type_014_typedef_handles() {
    let result = parse_type("typedef hkHandle<unsigned short,65535,hkndConstraintUniqueId>::<unnamed_tag> hkHandle<unsigned short,65535,hkSkinnedMeshShape::BoneSetIdDiscriminant>::<unnamed_tag>;");
    assert_eq!(
        result,
        vec!["hkHandle<unsigned short,65535,hkSkinnedMeshShape::BoneSetIdDiscriminant>", "<unnamed_tag>"]
    );
}

#[test]
fn test_type_015_enum_in_template() {
    let result = parse_type("enum hkcdDynamicTree::Tree<hkcdDynamicTree::DynamicStoragePtr>::<unnamed_tag> : __int32");
    assert_eq!(
        result,
        vec!["hkcdDynamicTree", "Tree<hkcdDynamicTree::DynamicStoragePtr>", "<unnamed_tag>"]
    );
}

#[test]
fn test_type_016_enum_simple_case() {
    let result = parse_type("enum hkpWorld::ReintegrationRecollideMode : __int32");
    assert_eq!(result, vec!["hkpWorld", "ReintegrationRecollideMode"]);
}

#[test]
fn test_type_017_enum_scaleform() {
    let result = parse_type("enum Scaleform::Render::Text::HTMLElementsEnum : __int32");
    assert_eq!(result, vec!["Scaleform", "Render", "Text", "HTMLElementsEnum"]);
}

#[test]
fn test_type_018_enum_d2d1() {
    let result = parse_type("enum D2D1_PATH_SEGMENT : __int32");
    assert_eq!(result, vec!["D2D1_PATH_SEGMENT"]);
}

#[test]
fn test_type_019_enum_force_pulse() {
    let result = parse_type("enum CForcePulse::E_MECH_PUNCH_DIR : __int32");
    assert_eq!(result, vec!["CForcePulse", "E_MECH_PUNCH_DIR"]);
}

#[test]
fn test_type_020_struct_with_align_and_base() {
    let result = parse_type("struct __cppobj __declspec(align(8)) std::_Hash<std::tr1::_Umap_traits<CCallContext const *,std::vector<boost::shared_ptr<CBaseUser const >> *,std::_Hash_compare<CCallContext const *,std::hash<CCallContext const *>,std::equal_to<CCallContext const *> >,std::allocator<std::pair<CCallContext const * const,std::vector<boost::shared_ptr<CBaseUser const >> *> >,0> > : std::tr1::_Umap_traits<CCallContext const *,std::vector<boost::shared_ptr<CBaseUser const >> *,std::_Hash_compare<CCallContext const *,std::hash<CCallContext const *>,std::equal_to<CCallContext const *> >,std::allocator<std::pair<CCallContext const * const,std::vector<boost::shared_ptr<CBaseUser const >> *> >,0>");
    assert_eq!(
        result,
        vec![
            "std",
            "_Hash<std::tr1::_Umap_traits<CCallContext const *,std::vector<boost::shared_ptr<CBaseUser const >> *,std::_Hash_compare<CCallContext const *,std::hash<CCallContext const *>,std::equal_to<CCallContext const *> >,std::allocator<std::pair<CCallContext const * const,std::vector<boost::shared_ptr<CBaseUser const >> *> >,0> >"
        ]
    );
}

#[test]
fn test_type_021_struct_with_allocator_base() {
    let result = parse_type("struct __cppobj boost::container::container_detail::vector_alloc_holder<CStatisticContext::allocator_t<std::pair<CHashString,CHashString> >,boost::container::container_detail::integral_constant<unsigned int,1> > : CStatisticContext::allocator_t<std::pair<CHashString,CHashString> >");
    assert_eq!(
        result,
        vec![
            "boost",
            "container",
            "container_detail",
            "vector_alloc_holder<CStatisticContext::allocator_t<std::pair<CHashString,CHashString> >,boost::container::container_detail::integral_constant<unsigned int,1> >"
        ]
    );
}

#[test]
fn test_type_022_simple_struct() {
    let result = parse_type("struct __cppobj IStatisticProvider::SEntry");
    assert_eq!(result, vec!["IStatisticProvider", "SEntry"]);
}

#[test]
fn test_type_023_const_struct_with_base() {
    let result = parse_type("const struct __cppobj OSuite::TKeyValueElement<OSuite::MetricPriorityKey,OSuitePrivate::InternalMetric *> : OSuite::TPair<OSuite::MetricPriorityKey,OSuitePrivate::InternalMetric *>");
    assert_eq!(
        result,
        vec![
            "OSuite",
            "TKeyValueElement<OSuite::MetricPriorityKey,OSuitePrivate::InternalMetric *>"
        ]
    );
}

#[test]
fn test_type_024_vtbl_struct() {
    let result = parse_type("struct /*VFT*/ OSuite::TList<OSuite::ZOEdmEntitySet *>_vtbl");
    assert_eq!(result, vec!["OSuite", "TList<OSuite::ZOEdmEntitySet *>_vtbl"]);
}

#[test]
fn test_type_025_simple_struct_hex_name() {
    let result = parse_type("struct $CABC8D72D33B996952E9BFEE2726C6EC");
    assert_eq!(result, vec!["$CABC8D72D33B996952E9BFEE2726C6EC"]);
}

#[test]
fn test_type_026_union_simple() {
    let result = parse_type("union __m128");
    assert_eq!(result, vec!["__m128"]);
}

#[test]
fn test_type_027_struct_hk_stream() {
    let result = parse_type("struct __cppobj hkBlockStreamBase::Stream");
    assert_eq!(result, vec!["hkBlockStreamBase", "Stream"]);
}

#[test]
fn test_type_028_struct_with_enum_template() {
    let result = parse_type("struct __cppobj hkEnum<enum hknpWorld::SimulationStage,unsigned int>");
    assert_eq!(result, vec!["hkEnum<enum hknpWorld::SimulationStage,unsigned int>"]);
}

#[test]
fn test_type_029_struct_array_with_base() {
    let result = parse_type("struct __cppobj hkArray<hknpDeactivatedIsland *,hkContainerHeapAllocator> : hkArrayBase<hknpDeactivatedIsland *>");
    assert_eq!(
        result,
        vec!["hkArray<hknpDeactivatedIsland *,hkContainerHeapAllocator>"]
    );
}

#[test]
fn test_type_030_struct_nested_unnamed() {
    let result = parse_type("struct TGrowableArray<SEffectInstanceData::SParam,8,0>::<unnamed_tag>::<unnamed_type_m_Aligner>");
    assert_eq!(
        result,
        vec![
            "TGrowableArray<SEffectInstanceData::SParam,8,0>",
            "<unnamed_tag>",
            "<unnamed_type_m_Aligner>"
        ]
    );
}

#[test]
fn test_type_031_struct_simple_handle() {
    let result = parse_type("struct Graphics::HVertexProgram_t");
    assert_eq!(result, vec!["Graphics", "HVertexProgram_t"]);
}

#[test]
fn test_type_032_struct_with_declspec_align() {
    let result = parse_type("struct __declspec(align(8)) SEffectRTParamHandler::ArrayParamHash");
    assert_eq!(result, vec!["SEffectRTParamHandler", "ArrayParamHash"]);
}

#[test]
fn test_type_033_struct_vector_val() {
    let result = parse_type("struct __cppobj __declspec(align(8)) std::_Vector_val<boost::shared_ptr<CAnimationInstance>> : std::_Container_base0");
    assert_eq!(result, vec!["std", "_Vector_val<boost::shared_ptr<CAnimationInstance>>"]);
}

#[test]
fn test_type_034_const_struct_with_base() {
    let result = parse_type("const struct __cppobj hkdGeometry : hkReferencedObject");
    assert_eq!(result, vec!["hkdGeometry"]);
}

#[test]
fn test_type_035_vtbl_wostream() {
    let result = parse_type("struct /*VFT*/ std::wostream_vtbl");
    assert_eq!(result, vec!["std", "wostream_vtbl"]);
}

#[test]
fn test_type_036_struct_exception() {
    let result = parse_type("struct __cppobj std::tr1::bad_weak_ptr : std::exception");
    assert_eq!(result, vec!["std", "tr1", "bad_weak_ptr"]);
}

#[test]
fn test_type_037_struct_lambda_template() {
    let result = parse_type("struct __cppobj TaskQueue::detail::CSyncWork<NSaveLoad::<lambda9>,void,bool> : TaskQueue::detail::CWork<NSaveLoad::<lambda9>,void,bool>");
    assert_eq!(
        result,
        vec![
            "TaskQueue",
            "detail",
            "CSyncWork<NSaveLoad::<lambda9>,void,bool>"
        ]
    );
}

#[test]
fn test_type_038_struct_hex_name_simple() {
    let result = parse_type("struct $B0414F1DEC363B3EFC1FC6E4B7F77361");
    assert_eq!(result, vec!["$B0414F1DEC363B3EFC1FC6E4B7F77361"]);
}

#[test]
fn test_type_039_struct_clone_tag() {
    let result = parse_type("struct __cppobj boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_weak_ptr> >::clone_tag");
    assert_eq!(
        result,
        vec![
            "boost",
            "exception_detail",
            "clone_impl<boost::exception_detail::error_info_injector<boost::bad_weak_ptr> >",
            "clone_tag"
        ]
    );
}

#[test]
fn test_type_040_struct_rebind_pointer() {
    let result = parse_type("struct __cppobj boost::intrusive::pointer_traits<std::pair<CCallContext const *,CStoreProvider_Steam::STransaction> *>::rebind_pointer<void>");
    assert_eq!(
        result,
        vec![
            "boost",
            "intrusive",
            "pointer_traits<std::pair<CCallContext const *,CStoreProvider_Steam::STransaction> *>",
            "rebind_pointer<void>"
        ]
    );
}

#[test]
fn test_type_041_union_callback_environ() {
    let result = parse_type("union _TP_CALLBACK_ENVIRON_V3::<unnamed_type_u>");
    assert_eq!(result, vec!["_TP_CALLBACK_ENVIRON_V3", "<unnamed_type_u>"]);
}

#[test]
fn test_type_042_struct_scaleform_array() {
    let result = parse_type("struct __cppobj Scaleform::ArrayBase<Scaleform::ArrayData<Scaleform::Ptr<Scaleform::GFx::ASIntervalTimerIntf>,Scaleform::AllocatorLH<Scaleform::Ptr<Scaleform::GFx::ASIntervalTimerIntf>,327>,Scaleform::ArrayDefaultPolicy> >");
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "ArrayBase<Scaleform::ArrayData<Scaleform::Ptr<Scaleform::GFx::ASIntervalTimerIntf>,Scaleform::AllocatorLH<Scaleform::Ptr<Scaleform::GFx::ASIntervalTimerIntf>,327>,Scaleform::ArrayDefaultPolicy> >"
        ]
    );
}

#[test]
fn test_type_043_struct_simple_no_qualifiers() {
    let result = parse_type("struct SEffectAttachment");
    assert_eq!(result, vec!["SEffectAttachment"]);
}

#[test]
fn test_type_044_struct_character_effect() {
    let result = parse_type("struct __cppobj __declspec(align(8)) CCharacter::SMeleeLoopEffect");
    assert_eq!(result, vec!["CCharacter", "SMeleeLoopEffect"]);
}

#[test]
fn test_type_045_struct_lambda() {
    let result = parse_type("struct __cppobj <lambda340>");
    assert_eq!(result, vec!["<lambda340>"]);
}

#[test]
fn test_type_046_typedef_enum_simple() {
    let result = parse_type("typedef hknpActivationMode::Enum hknpActivationBehavior::Enum;");
    assert_eq!(result, vec!["hknpActivationBehavior", "Enum"]);
}

#[test]
fn test_type_047_struct_no_namespace() {
    let result = parse_type("struct SLandSteering");
    assert_eq!(result, vec!["SLandSteering"]);
}

#[test]
fn test_type_048_struct_input_device() {
    let result = parse_type("struct Input::SDeviceType");
    assert_eq!(result, vec!["Input", "SDeviceType"]);
}

#[test]
fn test_type_049_struct_offer_item() {
    let result = parse_type("struct SOfferItem");
    assert_eq!(result, vec!["SOfferItem"]);
}

#[test]
fn test_type_050_struct_gear() {
    let result = parse_type("struct SGear");
    assert_eq!(result, vec!["SGear"]);
}

#[test]
fn test_type_051_const_struct_vehicle_lights() {
    let result = parse_type("const struct SVehicleDynamicLights");
    assert_eq!(result, vec!["SVehicleDynamicLights"]);
}

#[test]
fn test_type_052_struct_nongraphics_render_block() {
    let result = parse_type("struct __cppobj NGraphicsEngine::CRenderBlockBavariumShield::CRenderBlockTypeBavariumShield : NGraphicsEngine::CRenderBlockType");
    assert_eq!(
        result,
        vec![
            "NGraphicsEngine",
            "CRenderBlockBavariumShield",
            "CRenderBlockTypeBavariumShield"
        ]
    );
}

#[test]
fn test_type_053_enum_texture_cache() {
    let result = parse_type("enum ETextureCacheResult : __int32");
    assert_eq!(result, vec!["ETextureCacheResult"]);
}

#[test]
fn test_type_054_enum_ui_base_state() {
    let result = parse_type("enum CUIBase::State : __int32");
    assert_eq!(result, vec!["CUIBase", "State"]);
}
