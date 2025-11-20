use crate::type_parser::parse_type;

#[test]
fn test_type_001() {
    let result = parse_type("const enum __bitmask TmOption : __int32");
    assert_eq!(result, vec!["TmOption"]);
}

#[test]
fn test_type_002() {
    let result = parse_type("const struct __cppobj hkArrayBase<hkaiBehavior *>");
    assert_eq!(result, vec!["hkArrayBase<hkaiBehavior *>"]);
}

#[test]
fn test_type_003() {
    let result = parse_type("const struct __cppobj hkdGeometry : hkReferencedObject");
    assert_eq!(result, vec!["hkdGeometry"]);
}

#[test]
fn test_type_004() {
    let result = parse_type(
        "const struct __cppobj OSuite::TKeyValueElement<OSuite::MetricPriorityKey,OSuitePrivate::InternalMetric *> : OSuite::TPair<OSuite::MetricPriorityKey,OSuitePrivate::InternalMetric *>",
    );
    assert_eq!(
        result,
        vec![
            "OSuite",
            "TKeyValueElement<OSuite::MetricPriorityKey,OSuitePrivate::InternalMetric *>"
        ]
    );
}

#[test]
fn test_type_005() {
    let result = parse_type(
        "const struct __cppobj Scaleform::GFx::AS3::ClassTraits::Traits : Scaleform::GFx::AS3::Traits",
    );
    assert_eq!(
        result,
        vec!["Scaleform", "GFx", "AS3", "ClassTraits", "Traits"]
    );
}

#[test]
fn test_type_006() {
    let result = parse_type("const struct __declspec(align(8)) D2D1_HWND_RENDER_TARGET_PROPERTIES");
    assert_eq!(result, vec!["D2D1_HWND_RENDER_TARGET_PROPERTIES"]);
}

#[test]
fn test_type_007() {
    let result = parse_type("const struct D3D11_UNORDERED_ACCESS_VIEW_DESC");
    assert_eq!(result, vec!["D3D11_UNORDERED_ACCESS_VIEW_DESC"]);
}

#[test]
fn test_type_008() {
    let result = parse_type("const struct SVehicleDynamicLights");
    assert_eq!(result, vec!["SVehicleDynamicLights"]);
}

#[test]
fn test_type_009() {
    let result = parse_type("enum _KTMOBJECT_TYPE : __int32");
    assert_eq!(result, vec!["_KTMOBJECT_TYPE"]);
}

#[test]
fn test_type_010() {
    let result = parse_type("enum CAFSMDebugProtocol::EAnimationTrackMode : __int32");
    assert_eq!(result, vec!["CAFSMDebugProtocol", "EAnimationTrackMode"]);
}

#[test]
fn test_type_011() {
    let result = parse_type("enum CForcePulse::E_MECH_PUNCH_DIR : __int32");
    assert_eq!(result, vec!["CForcePulse", "E_MECH_PUNCH_DIR"]);
}

#[test]
fn test_type_012() {
    let result = parse_type("enum CGrapplePointProxy::EAttachmentObjectInfo : __int32");
    assert_eq!(result, vec!["CGrapplePointProxy", "EAttachmentObjectInfo"]);
}

#[test]
fn test_type_013() {
    let result =
        parse_type("enum CMeshEffectManager::CreateMeshParticle::__l2::EMeshParams : __int32");
    assert_eq!(
        result,
        vec![
            "CMeshEffectManager",
            "CreateMeshParticle",
            "__l2",
            "EMeshParams"
        ]
    );
}

#[test]
fn test_type_014() {
    let result = parse_type("enum CrankcaseAudio::eRampType : __int32");
    assert_eq!(result, vec!["CrankcaseAudio", "eRampType"]);
}

#[test]
fn test_type_015() {
    let result = parse_type("enum CUIBase::State : __int32");
    assert_eq!(result, vec!["CUIBase", "State"]);
}

#[test]
fn test_type_016() {
    let result = parse_type("enum CWeaponBase::EDebugWeaponDetailedInfo : __int32");
    assert_eq!(result, vec!["CWeaponBase", "EDebugWeaponDetailedInfo"]);
}

#[test]
fn test_type_017() {
    let result = parse_type("enum D2D1_PATH_SEGMENT : __int32");
    assert_eq!(result, vec!["D2D1_PATH_SEGMENT"]);
}

#[test]
fn test_type_018() {
    let result = parse_type("enum EAIFactionGroup : __int32");
    assert_eq!(result, vec!["EAIFactionGroup"]);
}

#[test]
fn test_type_019() {
    let result = parse_type("enum EMemoryRange : __int32");
    assert_eq!(result, vec!["EMemoryRange"]);
}

#[test]
fn test_type_020() {
    let result = parse_type("enum ETextureCacheResult : __int32");
    assert_eq!(result, vec!["ETextureCacheResult"]);
}

#[test]
fn test_type_021() {
    let result = parse_type("enum EVoiceResult : __int32");
    assert_eq!(result, vec!["EVoiceResult"]);
}

#[test]
fn test_type_022() {
    let result = parse_type("enum hkaiDegenerateFaceCutter::<unnamed_tag> : __int32");
    assert_eq!(result, vec!["hkaiDegenerateFaceCutter", "<unnamed_tag>"]);
}

#[test]
fn test_type_023() {
    let result = parse_type("enum hkaiNavMeshCutter::<unnamed_tag> : __int32");
    assert_eq!(result, vec!["hkaiNavMeshCutter", "<unnamed_tag>"]);
}

#[test]
fn test_type_024() {
    let result = parse_type(
        "enum hkcdDynamicTree::Tree<hkcdDynamicTree::DynamicStoragePtr>::<unnamed_tag> : __int32",
    );
    assert_eq!(
        result,
        vec![
            "hkcdDynamicTree",
            "Tree<hkcdDynamicTree::DynamicStoragePtr>",
            "<unnamed_tag>"
        ]
    );
}

#[test]
fn test_type_025() {
    let result = parse_type("enum hkdAction::ReplaceType : __int32");
    assert_eq!(result, vec!["hkdAction", "ReplaceType"]);
}

#[test]
fn test_type_026() {
    let result = parse_type("enum hkndIBFracWireBuilder::WirePoint::Flags : __int32");
    assert_eq!(result, vec!["hkndIBFracWireBuilder", "WirePoint", "Flags"]);
}

#[test]
fn test_type_027() {
    let result = parse_type("enum hkpConvexShape::WeldResult : __int32");
    assert_eq!(result, vec!["hkpConvexShape", "WeldResult"]);
}

#[test]
fn test_type_028() {
    let result = parse_type("enum hkpWorld::ReintegrationRecollideMode : __int32");
    assert_eq!(result, vec!["hkpWorld", "ReintegrationRecollideMode"]);
}

#[test]
fn test_type_029() {
    let result = parse_type("enum Input::InputSystemBlocker : __int32");
    assert_eq!(result, vec!["Input", "InputSystemBlocker"]);
}

#[test]
fn test_type_030() {
    let result = parse_type("enum NAnimationSystem::CAnimationTrack::EPhaseMatchMode : __int32");
    assert_eq!(
        result,
        vec!["NAnimationSystem", "CAnimationTrack", "EPhaseMatchMode"]
    );
}

#[test]
fn test_type_031() {
    let result =
        parse_type("enum NAttachedEffectContainer::SAttachedInstance::ESoundLayerState : __int32");
    assert_eq!(
        result,
        vec![
            "NAttachedEffectContainer",
            "SAttachedInstance",
            "ESoundLayerState"
        ]
    );
}

#[test]
fn test_type_032() {
    let result = parse_type("enum NGraphicsEngine::CWindowContactPointList::EDamageType : __int32");
    assert_eq!(
        result,
        vec!["NGraphicsEngine", "CWindowContactPointList", "EDamageType"]
    );
}

#[test]
fn test_type_033() {
    let result = parse_type("enum NGSONodes::IsUpright::__l2::<unnamed_tag> : __int32");
    assert_eq!(
        result,
        vec!["NGSONodes", "IsUpright", "__l2", "<unnamed_tag>"]
    );
}

#[test]
fn test_type_034() {
    let result = parse_type("enum NGSONodes::TriggerWingsuit::__l2::EFields : __int32");
    assert_eq!(
        result,
        vec!["NGSONodes", "TriggerWingsuit", "__l2", "EFields"]
    );
}

#[test]
fn test_type_035() {
    let result = parse_type("enum NParatrooperLogic::EState : __int32");
    assert_eq!(result, vec!["NParatrooperLogic", "EState"]);
}

#[test]
fn test_type_036() {
    let result = parse_type("enum NVehicle::EDoorSlot : __int32");
    assert_eq!(result, vec!["NVehicle", "EDoorSlot"]);
}

#[test]
fn test_type_037() {
    let result = parse_type("enum Scaleform::GFx::AS2::Environment::<unnamed_tag> : __int32");
    assert_eq!(
        result,
        vec!["Scaleform", "GFx", "AS2", "Environment", "<unnamed_tag>"]
    );
}

#[test]
fn test_type_038() {
    let result = parse_type(
        "enum Scaleform::GFx::AS3::VM::exec_typebarrier_reg::__l18::<unnamed_tag> : __int32",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "GFx",
            "AS3",
            "VM",
            "exec_typebarrier_reg",
            "__l18",
            "<unnamed_tag>"
        ]
    );
}

#[test]
fn test_type_039() {
    let result = parse_type("enum Scaleform::GFx::DisplayObjectBase::TopMostResult : __int32");
    assert_eq!(
        result,
        vec!["Scaleform", "GFx", "DisplayObjectBase", "TopMostResult"]
    );
}

#[test]
fn test_type_040() {
    let result = parse_type("enum Scaleform::GFx::TextKeyMap::KeyState : __int32");
    assert_eq!(result, vec!["Scaleform", "GFx", "TextKeyMap", "KeyState"]);
}

#[test]
fn test_type_041() {
    let result =
        parse_type("enum Scaleform::MsgFormat::Parse::__l2::<unnamed_type_state> : __int32");
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "MsgFormat",
            "Parse",
            "__l2",
            "<unnamed_type_state>"
        ]
    );
}

#[test]
fn test_type_042() {
    let result = parse_type("enum Scaleform::Render::PrimitiveFillFlags : __int32");
    assert_eq!(result, vec!["Scaleform", "Render", "PrimitiveFillFlags"]);
}

#[test]
fn test_type_043() {
    let result = parse_type("enum Scaleform::Render::SortKeyTransition : __int32");
    assert_eq!(result, vec!["Scaleform", "Render", "SortKeyTransition"]);
}

#[test]
fn test_type_044() {
    let result = parse_type("enum Scaleform::Render::Text::HTMLElementsEnum : __int32");
    assert_eq!(
        result,
        vec!["Scaleform", "Render", "Text", "HTMLElementsEnum"]
    );
}

#[test]
fn test_type_045() {
    let result = parse_type("enum SProfileOffer::OfferType : __int32");
    assert_eq!(result, vec!["SProfileOffer", "OfferType"]);
}

#[test]
fn test_type_046() {
    let result = parse_type("enum std::stringbuf::<unnamed_tag> : __int32");
    assert_eq!(result, vec!["std", "stringbuf", "<unnamed_tag>"]);
}

#[test]
fn test_type_047() {
    let result = parse_type(
        "struct __cppobj __declspec(align(16)) CCharacterManager : Base::IAppSystem, Base::CSingle<CCharacterManager>",
    );
    assert_eq!(result, vec!["CCharacterManager"]);
}

#[test]
fn test_type_048() {
    let result =
        parse_type("struct __cppobj __declspec(align(16)) hkDisplayMesh : hkDisplayGeometry");
    assert_eq!(result, vec!["hkDisplayMesh"]);
}

#[test]
fn test_type_049() {
    let result = parse_type(
        "struct __cppobj __declspec(align(16)) hkndIntegrityAnalyzerRuntimeImpl::DisableConns",
    );
    assert_eq!(
        result,
        vec!["hkndIntegrityAnalyzerRuntimeImpl", "DisableConns"]
    );
}

#[test]
fn test_type_050() {
    let result =
        parse_type("struct __cppobj __declspec(align(16)) hkSimpleLocalFrame : hkLocalFrame");
    assert_eq!(result, vec!["hkSimpleLocalFrame"]);
}

#[test]
fn test_type_051() {
    let result = parse_type("struct __cppobj __declspec(align(2)) hkgpConvexHullImpl");
    assert_eq!(result, vec!["hkgpConvexHullImpl"]);
}

#[test]
fn test_type_052() {
    let result = parse_type("struct __cppobj __declspec(align(4)) CSungunWeapon : CWeaponBase");
    assert_eq!(result, vec!["CSungunWeapon"]);
}

#[test]
fn test_type_053() {
    let result =
        parse_type("struct __cppobj __declspec(align(4)) hknpAddBodyCommand : hknpApiCommand");
    assert_eq!(result, vec!["hknpAddBodyCommand"]);
}

#[test]
fn test_type_054() {
    let result = parse_type(
        "struct __cppobj __declspec(align(4)) Scaleform::GFx::AS3::Instances::fl_ui::ContextMenuClipboardItems : Scaleform::GFx::AS3::Instances::fl::Object",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "GFx",
            "AS3",
            "Instances",
            "fl_ui",
            "ContextMenuClipboardItems"
        ]
    );
}

#[test]
fn test_type_055() {
    let result = parse_type(
        "struct __cppobj __declspec(align(4)) Scaleform::GFx::TouchEventId : Scaleform::GFx::EventId",
    );
    assert_eq!(result, vec!["Scaleform", "GFx", "TouchEventId"]);
}

#[test]
fn test_type_056() {
    let result = parse_type(
        "struct __cppobj __declspec(align(8)) boost::detail::sp_counted_impl_pd<CBoneOffsetAlias *,CGameObjectCreator<CBoneOffsetAlias>::SGameObjectDestroyer> : boost::detail::sp_counted_base",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "detail",
            "sp_counted_impl_pd<CBoneOffsetAlias *,CGameObjectCreator<CBoneOffsetAlias>::SGameObjectDestroyer>"
        ]
    );
}

#[test]
fn test_type_057() {
    let result = parse_type(
        "struct __cppobj __declspec(align(8)) boost::detail::sp_counted_impl_pd<CCharacter *,CGameObjectCreator<CCharacter>::SGameObjectDestroyer> : boost::detail::sp_counted_base",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "detail",
            "sp_counted_impl_pd<CCharacter *,CGameObjectCreator<CCharacter>::SGameObjectDestroyer>"
        ]
    );
}

#[test]
fn test_type_058() {
    let result = parse_type(
        "struct __cppobj __declspec(align(8)) boost::detail::sp_counted_impl_pd<COnlinePlatformSystemObserver *,COnlineObserver<COnlinePlatformSystemObserver>::null_deleter> : boost::detail::sp_counted_base",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "detail",
            "sp_counted_impl_pd<COnlinePlatformSystemObserver *,COnlineObserver<COnlinePlatformSystemObserver>::null_deleter>"
        ]
    );
}

#[test]
fn test_type_059() {
    let result = parse_type(
        "struct __cppobj __declspec(align(8)) boost::detail::sp_counted_impl_pd<CStaticDecalObject *,CGameObjectCreator<CStaticDecalObject>::SGameObjectDestroyer> : boost::detail::sp_counted_base",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "detail",
            "sp_counted_impl_pd<CStaticDecalObject *,CGameObjectCreator<CStaticDecalObject>::SGameObjectDestroyer>"
        ]
    );
}

#[test]
fn test_type_060() {
    let result = parse_type(
        "struct __cppobj __declspec(align(8)) boost::detail::sp_counted_impl_pd<CTargetConnector *,CGameObjectCreator<CTargetConnector>::SGameObjectDestroyer> : boost::detail::sp_counted_base",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "detail",
            "sp_counted_impl_pd<CTargetConnector *,CGameObjectCreator<CTargetConnector>::SGameObjectDestroyer>"
        ]
    );
}

#[test]
fn test_type_061() {
    let result = parse_type(
        "struct __cppobj __declspec(align(8)) CAny::ValueHolder<enum NOnline::EErrorCode> : CAny::Placeholder",
    );
    assert_eq!(
        result,
        vec!["CAny", "ValueHolder<enum NOnline::EErrorCode>"]
    );
}

#[test]
fn test_type_062() {
    let result = parse_type(
        "struct __cppobj __declspec(align(8)) CCameraConditionRotation : CCameraCondition",
    );
    assert_eq!(result, vec!["CCameraConditionRotation"]);
}

#[test]
fn test_type_063() {
    let result = parse_type("struct __cppobj __declspec(align(8)) CCharacter::SMeleeLoopEffect");
    assert_eq!(result, vec!["CCharacter", "SMeleeLoopEffect"]);
}

#[test]
fn test_type_064() {
    let result = parse_type(
        "struct __cppobj __declspec(align(8)) CGraphEntryIsFacingNodeCondition : CInteractionGraphEntryAFSMC",
    );
    assert_eq!(result, vec!["CGraphEntryIsFacingNodeCondition"]);
}

#[test]
fn test_type_065() {
    let result = parse_type(
        "struct __cppobj __declspec(align(8)) CGraphEntryNodeDistanceCondition : CInteractionGraphEntryAFSMC",
    );
    assert_eq!(result, vec!["CGraphEntryNodeDistanceCondition"]);
}

#[test]
fn test_type_066() {
    let result = parse_type("struct __cppobj __declspec(align(8)) CHeatVocals : CVocalsBase");
    assert_eq!(result, vec!["CHeatVocals"]);
}

#[test]
fn test_type_067() {
    let result = parse_type(
        "struct __cppobj __declspec(align(8)) CRandomSuccessChance : NAnimationSystem::CCondition",
    );
    assert_eq!(result, vec!["CRandomSuccessChance"]);
}

#[test]
fn test_type_068() {
    let result = parse_type(
        "struct __cppobj __declspec(align(8)) Graphics::CustomAllocator<std::_Tree_nod<std::_Tmap_traits<unsigned __int64,ID3D11DepthStencilState *,std::less<unsigned __int64>,Graphics::CustomAllocator<std::pair<unsigned __int64 const ,ID3D11DepthStencilState *>,14>,0> >::_Node,14> : std::allocator<std::_Tree_nod<std::_Tmap_traits<unsigned __int64,ID3D11DepthStencilState *,std::less<unsigned __int64>,Graphics::CustomAllocator<std::pair<unsigned __int64 const ,ID3D11DepthStencilState *>,14>,0> >::_Node>",
    );
    assert_eq!(
        result,
        vec![
            "Graphics",
            "CustomAllocator<std::_Tree_nod<std::_Tmap_traits<unsigned __int64,ID3D11DepthStencilState *,std::less<unsigned __int64>,Graphics::CustomAllocator<std::pair<unsigned __int64 const ,ID3D11DepthStencilState *>,14>,0> >::_Node,14>"
        ]
    );
}

#[test]
fn test_type_069() {
    let result =
        parse_type("struct __cppobj __declspec(align(8)) hkFixedArray<hk1AxisSweep::AabbInt>");
    assert_eq!(result, vec!["hkFixedArray<hk1AxisSweep::AabbInt>"]);
}

#[test]
fn test_type_070() {
    let result = parse_type(
        "struct __cppobj __declspec(align(8)) hkFixedArray<hkHandle<unsigned int,536870911,hkcdPlanarGeometryPolygonCollection::VertexIdDiscriminant> >",
    );
    assert_eq!(
        result,
        vec![
            "hkFixedArray<hkHandle<unsigned int,536870911,hkcdPlanarGeometryPolygonCollection::VertexIdDiscriminant> >"
        ]
    );
}

#[test]
fn test_type_071() {
    let result = parse_type(
        "struct __cppobj __declspec(align(8)) hkLocalArray<hkTaskScheduler::TaskDepth> : hkArray<hkTaskScheduler::TaskDepth,hkContainerHeapAllocator>",
    );
    assert_eq!(result, vec!["hkLocalArray<hkTaskScheduler::TaskDepth>"]);
}

#[test]
fn test_type_072() {
    let result = parse_type(
        "struct __cppobj __declspec(align(8)) hkLocalArray<unsigned char> : hkArray<unsigned char,hkContainerHeapAllocator>",
    );
    assert_eq!(result, vec!["hkLocalArray<unsigned char>"]);
}

#[test]
fn test_type_073() {
    let result = parse_type("struct __cppobj __declspec(align(8)) hkpCdPointCollector");
    assert_eq!(result, vec!["hkpCdPointCollector"]);
}

#[test]
fn test_type_074() {
    let result =
        parse_type("struct __cppobj __declspec(align(8)) hkpMoppAabbJob : hkpCollisionQueryJob");
    assert_eq!(result, vec!["hkpMoppAabbJob"]);
}

#[test]
fn test_type_075() {
    let result = parse_type(
        "struct __cppobj __declspec(align(8)) NAnimationSystem::CSpecificCreator<NAnimationSystem::CBoxBase,NAnimationSystem::CModuloBox> : NAnimationSystem::CCreatorBase<NAnimationSystem::CBoxBase>",
    );
    assert_eq!(
        result,
        vec![
            "NAnimationSystem",
            "CSpecificCreator<NAnimationSystem::CBoxBase,NAnimationSystem::CModuloBox>"
        ]
    );
}

#[test]
fn test_type_076() {
    let result = parse_type(
        "struct __cppobj __declspec(align(8)) NAr::detail::CLoadSchemaVisitor<NAr::CBlobStream>",
    );
    assert_eq!(
        result,
        vec!["NAr", "detail", "CLoadSchemaVisitor<NAr::CBlobStream>"]
    );
}

#[test]
fn test_type_077() {
    let result = parse_type(
        "struct __cppobj __declspec(align(8)) NPfxBreakable::SDeferredAngularImpulseApplicator_World<CPfxBreakable,2315920433>",
    );
    assert_eq!(
        result,
        vec![
            "NPfxBreakable",
            "SDeferredAngularImpulseApplicator_World<CPfxBreakable,2315920433>"
        ]
    );
}

#[test]
fn test_type_078() {
    let result = parse_type(
        "struct __cppobj __declspec(align(8)) Scaleform::Formatter : Scaleform::FmtResource, Unassignable",
    );
    assert_eq!(result, vec!["Scaleform", "Formatter"]);
}

#[test]
fn test_type_079() {
    let result = parse_type(
        "struct __cppobj __declspec(align(8)) Scaleform::GFx::AS3::UnboxArgV3NC<Scaleform::GFx::AS3::Value const ,Scaleform::GFx::AS3::Instances::fl_utils::ByteArray *,unsigned long,unsigned long> : Scaleform::GFx::AS3::UnboxArgV2NC<Scaleform::GFx::AS3::Value const ,Scaleform::GFx::AS3::Instances::fl_utils::ByteArray *,unsigned long>",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "GFx",
            "AS3",
            "UnboxArgV3NC<Scaleform::GFx::AS3::Value const ,Scaleform::GFx::AS3::Instances::fl_utils::ByteArray *,unsigned long,unsigned long>"
        ]
    );
}

#[test]
fn test_type_080() {
    let result = parse_type(
        "struct __cppobj __declspec(align(8)) Scaleform::GFx::AS3::UnboxArgV4NC<Scaleform::GFx::AS3::SPtr<Scaleform::GFx::AS3::Instances::fl_net::SharedObject>,Scaleform::GFx::ASString const &,Scaleform::GFx::ASString const &,Scaleform::GFx::AS3::Value const &,bool> : Scaleform::GFx::AS3::UnboxArgV3NC<Scaleform::GFx::AS3::SPtr<Scaleform::GFx::AS3::Instances::fl_net::SharedObject>,Scaleform::GFx::ASString const &,Scaleform::GFx::ASString const &,Scaleform::GFx::AS3::Value const &>",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "GFx",
            "AS3",
            "UnboxArgV4NC<Scaleform::GFx::AS3::SPtr<Scaleform::GFx::AS3::Instances::fl_net::SharedObject>,Scaleform::GFx::ASString const &,Scaleform::GFx::ASString const &,Scaleform::GFx::AS3::Value const &,bool>"
        ]
    );
}

#[test]
fn test_type_081() {
    let result = parse_type(
        "struct __cppobj __declspec(align(8)) Scaleform::GFx::AS3::UnboxArgV4NC<Scaleform::GFx::AS3::Value const ,Scaleform::GFx::ASString const &,Scaleform::GFx::AS3::Value const &,bool,long> : Scaleform::GFx::AS3::UnboxArgV3NC<Scaleform::GFx::AS3::Value const ,Scaleform::GFx::ASString const &,Scaleform::GFx::AS3::Value const &,bool>",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "GFx",
            "AS3",
            "UnboxArgV4NC<Scaleform::GFx::AS3::Value const ,Scaleform::GFx::ASString const &,Scaleform::GFx::AS3::Value const &,bool,long>"
        ]
    );
}

#[test]
fn test_type_082() {
    let result = parse_type(
        "struct __cppobj __declspec(align(8)) Scaleform::Render::SIF::SIFFileImageSource : Scaleform::Render::FileImageSource",
    );
    assert_eq!(
        result,
        vec!["Scaleform", "Render", "SIF", "SIFFileImageSource"]
    );
}

#[test]
fn test_type_083() {
    let result = parse_type("struct __cppobj __declspec(align(8)) SPatchSystemUpdateDesc");
    assert_eq!(result, vec!["SPatchSystemUpdateDesc"]);
}

#[test]
fn test_type_084() {
    let result = parse_type(
        "struct __cppobj __declspec(align(8)) std::_Hash<std::tr1::_Umap_traits<CCallContext const *,std::vector<boost::shared_ptr<CBaseUser const >> *,std::_Hash_compare<CCallContext const *,std::hash<CCallContext const *>,std::equal_to<CCallContext const *> >,std::allocator<std::pair<CCallContext const * const,std::vector<boost::shared_ptr<CBaseUser const >> *> >,0> > : std::tr1::_Umap_traits<CCallContext const *,std::vector<boost::shared_ptr<CBaseUser const >> *,std::_Hash_compare<CCallContext const *,std::hash<CCallContext const *>,std::equal_to<CCallContext const *> >,std::allocator<std::pair<CCallContext const * const,std::vector<boost::shared_ptr<CBaseUser const >> *> >,0>",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "_Hash<std::tr1::_Umap_traits<CCallContext const *,std::vector<boost::shared_ptr<CBaseUser const >> *,std::_Hash_compare<CCallContext const *,std::hash<CCallContext const *>,std::equal_to<CCallContext const *> >,std::allocator<std::pair<CCallContext const * const,std::vector<boost::shared_ptr<CBaseUser const >> *> >,0> >"
        ]
    );
}

#[test]
fn test_type_085() {
    let result = parse_type(
        "struct __cppobj __declspec(align(8)) std::_List_nod<CUIBase::Command> : std::_Container_base0",
    );
    assert_eq!(result, vec!["std", "_List_nod<CUIBase::Command>"]);
}

#[test]
fn test_type_086() {
    let result = parse_type(
        "struct __cppobj __declspec(align(8)) std::_Pair_base<boost::container::container_detail::pair<ava::idstring<PropertyContainerIdStringTag,1>,TPropertyContainer<ava::idstring<PropertyContainerIdStringTag,1>,std::allocator<int>,CIDStringHash> > *,bool>",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "_Pair_base<boost::container::container_detail::pair<ava::idstring<PropertyContainerIdStringTag,1>,TPropertyContainer<ava::idstring<PropertyContainerIdStringTag,1>,std::allocator<int>,CIDStringHash> > *,bool>"
        ]
    );
}

#[test]
fn test_type_087() {
    let result =
        parse_type("struct __cppobj __declspec(align(8)) std::_Pair_base<CHashString,int>");
    assert_eq!(result, vec!["std", "_Pair_base<CHashString,int>"]);
}

#[test]
fn test_type_088() {
    let result = parse_type(
        "struct __cppobj __declspec(align(8)) std::_Pair_base<unsigned __int64 const ,CVdbShapeDisplayHandler::SShapeDisplayGeometry>",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "_Pair_base<unsigned __int64 const ,CVdbShapeDisplayHandler::SShapeDisplayGeometry>"
        ]
    );
}

#[test]
fn test_type_089() {
    let result = parse_type(
        "struct __cppobj __declspec(align(8)) std::_Tree_nod<std::_Tmap_traits<unsigned int,std::vector<NADFSubscription::ADFSubscriber *>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const ,std::vector<NADFSubscription::ADFSubscriber *> > >,0> >::_Node",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "_Tree_nod<std::_Tmap_traits<unsigned int,std::vector<NADFSubscription::ADFSubscriber *>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const ,std::vector<NADFSubscription::ADFSubscriber *> > >,0> >",
            "_Node"
        ]
    );
}

#[test]
fn test_type_090() {
    let result = parse_type(
        "struct __cppobj __declspec(align(8)) std::_Vector_val<boost::shared_ptr<CAnimationInstance>> : std::_Container_base0",
    );
    assert_eq!(
        result,
        vec!["std", "_Vector_val<boost::shared_ptr<CAnimationInstance>>"]
    );
}

#[test]
fn test_type_091() {
    let result = parse_type(
        "struct __cppobj __declspec(align(8)) std::_Vector_val<CBatchRegisterEvents::UMemoryBlock> : std::_Container_base0",
    );
    assert_eq!(
        result,
        vec!["std", "_Vector_val<CBatchRegisterEvents::UMemoryBlock>"]
    );
}

#[test]
fn test_type_092() {
    let result = parse_type(
        "struct __cppobj __declspec(align(8)) std::_Vector_val<CSpawnSystem::SBudgetRecord> : std::_Container_base0",
    );
    assert_eq!(
        result,
        vec!["std", "_Vector_val<CSpawnSystem::SBudgetRecord>"]
    );
}

#[test]
fn test_type_093() {
    let result = parse_type(
        "struct __cppobj __declspec(align(8)) std::_Vector_val<CStatisticMarshaler::SRegisterInfo<TDelegate<void __cdecl(CStatistic<__int64> const *,__int64,__int64,CStatisticContext const &)> >> : std::_Container_base0",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "_Vector_val<CStatisticMarshaler::SRegisterInfo<TDelegate<void __cdecl(CStatistic<__int64> const *,__int64,__int64,CStatisticContext const &)> >>"
        ]
    );
}

#[test]
fn test_type_094() {
    let result = parse_type(
        "struct __cppobj __declspec(align(8)) std::_Vector_val<NVehicle_Debug::SDestroyedVehicleLog> : std::_Container_base0",
    );
    assert_eq!(
        result,
        vec!["std", "_Vector_val<NVehicle_Debug::SDestroyedVehicleLog>"]
    );
}

#[test]
fn test_type_095() {
    let result = parse_type(
        "struct __cppobj __declspec(align(8)) std::_Vector_val<STrafficZone *> : std::_Container_base0",
    );
    assert_eq!(result, vec!["std", "_Vector_val<STrafficZone *>"]);
}

#[test]
fn test_type_096() {
    let result = parse_type(
        "struct __cppobj __declspec(align(8)) std::_Vector_val<testing::Environment *> : std::_Container_base0",
    );
    assert_eq!(result, vec!["std", "_Vector_val<testing::Environment *>"]);
}

#[test]
fn test_type_097() {
    let result = parse_type(
        "struct __cppobj __declspec(align(8)) std::tr1::_Impl_no_alloc1<std::tr1::_Callable_obj<<lambda75>,0>,void,CCallContext &> : std::tr1::_Impl_base1<void,CCallContext &>",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "tr1",
            "_Impl_no_alloc1<std::tr1::_Callable_obj<<lambda75>,0>,void,CCallContext &>"
        ]
    );
}

#[test]
fn test_type_098() {
    let result = parse_type(
        "struct __cppobj __declspec(align(8)) std::tr1::_Impl_no_alloc1<std::tr1::_Callable_obj<A0x78becc47::<lambda38>,0>,void,CCallContextResult<STaskAttemptResult> &> : std::tr1::_Impl_base1<void,CCallContextResult<STaskAttemptResult> &>",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "tr1",
            "_Impl_no_alloc1<std::tr1::_Callable_obj<A0x78becc47::<lambda38>,0>,void,CCallContextResult<STaskAttemptResult> &>"
        ]
    );
}

#[test]
fn test_type_099() {
    let result =
        parse_type("struct __cppobj __declspec(align(8)) StreamContactSolver::WriterHelper<0>");
    assert_eq!(result, vec!["StreamContactSolver", "WriterHelper<0>"]);
}

#[test]
fn test_type_100() {
    let result = parse_type("struct __cppobj __declspec(align(8)) TArray<SEventID>");
    assert_eq!(result, vec!["TArray<SEventID>"]);
}

#[test]
fn test_type_101() {
    let result = parse_type(
        "struct __cppobj __declspec(align(8)) THashTable<CRicoVocals::STriggerState,unsigned int,1,unsigned short>",
    );
    assert_eq!(
        result,
        vec!["THashTable<CRicoVocals::STriggerState,unsigned int,1,unsigned short>"]
    );
}

#[test]
fn test_type_102() {
    let result = parse_type(
        "struct __cppobj __unaligned __declspec(align(8)) std::tr1::_Bind1<std::tr1::_Callable_pmf<void (__cdecl CDamageable::*const)(short,CDamageMsg *),CDamageable,0>,CVehicle *> : std::tr1::_Bind0<std::tr1::_Callable_pmf<void (__cdecl CDamageable::*const)(short,CDamageMsg *),CDamageable,0> >",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "tr1",
            "_Bind1<std::tr1::_Callable_pmf<void (__cdecl CDamageable::*const)(short,CDamageMsg *),CDamageable,0>,CVehicle *>"
        ]
    );
}

#[test]
fn test_type_103() {
    let result = parse_type("struct __cppobj __vc_attributes::iid_isAttribute");
    assert_eq!(result, vec!["__vc_attributes", "iid_isAttribute"]);
}

#[test]
fn test_type_104() {
    let result = parse_type("struct __cppobj __vc_attributes::odlAttribute");
    assert_eq!(result, vec!["__vc_attributes", "odlAttribute"]);
}

#[test]
fn test_type_105() {
    let result = parse_type("struct __cppobj <lambda193>");
    assert_eq!(result, vec!["<lambda193>"]);
}

#[test]
fn test_type_106() {
    let result = parse_type("struct __cppobj <lambda340>");
    assert_eq!(result, vec!["<lambda340>"]);
}

#[test]
fn test_type_107() {
    let result = parse_type("struct __cppobj <lambda349>");
    assert_eq!(result, vec!["<lambda349>"]);
}

#[test]
fn test_type_108() {
    let result = parse_type("struct __cppobj <lambda351>");
    assert_eq!(result, vec!["<lambda351>"]);
}

#[test]
fn test_type_109() {
    let result = parse_type(
        "struct __cppobj ATL::CComPtr<IConnectionPointContainer> : ATL::CComPtrBase<IConnectionPointContainer>",
    );
    assert_eq!(result, vec!["ATL", "CComPtr<IConnectionPointContainer>"]);
}

#[test]
fn test_type_110() {
    let result = parse_type("struct __cppobj Base::CSingle<CVegetationInteraction>");
    assert_eq!(result, vec!["Base", "CSingle<CVegetationInteraction>"]);
}

#[test]
fn test_type_111() {
    let result = parse_type(
        "struct __cppobj boost::concepts::usage_requirements<boost::EqualityComparable<std::_String_iterator<char,std::char_traits<char>,std::allocator<char> > > >",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "concepts",
            "usage_requirements<boost::EqualityComparable<std::_String_iterator<char,std::char_traits<char>,std::allocator<char> > > >"
        ]
    );
}

#[test]
fn test_type_112() {
    let result = parse_type(
        "struct __cppobj boost::container::allocator_traits<std::allocator<boost::container::container_detail::pair<CHashString,SBucketDefinition *> > >",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "container",
            "allocator_traits<std::allocator<boost::container::container_detail::pair<CHashString,SBucketDefinition *> > >"
        ]
    );
}

#[test]
fn test_type_113() {
    let result = parse_type(
        "struct __cppobj boost::container::allocator_traits<std::allocator<std::pair<CHashString,SProfileItemInfo> > >",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "container",
            "allocator_traits<std::allocator<std::pair<CHashString,SProfileItemInfo> > >"
        ]
    );
}

#[test]
fn test_type_114() {
    let result = parse_type(
        "struct __cppobj boost::container::container_detail::flat_tree<CCallContext const *,boost::container::container_detail::pair<CCallContext const *,boost::container::flat_map<int,SStoreItemInfo,std::less<int>,std::allocator<std::pair<int,SStoreItemInfo> > > *>,boost::container::container_detail::select1st<boost::container::container_detail::pair<CCallContext const *,boost::container::flat_map<int,SStoreItemInfo,std::less<int>,std::allocator<std::pair<int,SStoreItemInfo> > > *> >,std::less<CCallContext const *>,std::allocator<boost::container::container_detail::pair<CCallContext const *,boost::container::flat_map<int,SStoreItemInfo,std::less<int>,std::allocator<std::pair<int,SStoreItemInfo> > > *> > >",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "container",
            "container_detail",
            "flat_tree<CCallContext const *,boost::container::container_detail::pair<CCallContext const *,boost::container::flat_map<int,SStoreItemInfo,std::less<int>,std::allocator<std::pair<int,SStoreItemInfo> > > *>,boost::container::container_detail::select1st<boost::container::container_detail::pair<CCallContext const *,boost::container::flat_map<int,SStoreItemInfo,std::less<int>,std::allocator<std::pair<int,SStoreItemInfo> > > *> >,std::less<CCallContext const *>,std::allocator<boost::container::container_detail::pair<CCallContext const *,boost::container::flat_map<int,SStoreItemInfo,std::less<int>,std::allocator<std::pair<int,SStoreItemInfo> > > *> > >"
        ]
    );
}

#[test]
fn test_type_115() {
    let result = parse_type(
        "struct __cppobj boost::container::container_detail::flat_tree<CHashString,boost::container::container_detail::pair<CHashString,double>,boost::container::container_detail::select1st<boost::container::container_detail::pair<CHashString,double> >,std::less<CHashString>,std::allocator<boost::container::container_detail::pair<CHashString,double> > >",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "container",
            "container_detail",
            "flat_tree<CHashString,boost::container::container_detail::pair<CHashString,double>,boost::container::container_detail::select1st<boost::container::container_detail::pair<CHashString,double> >,std::less<CHashString>,std::allocator<boost::container::container_detail::pair<CHashString,double> > >"
        ]
    );
}

#[test]
fn test_type_116() {
    let result = parse_type(
        "struct __cppobj boost::container::container_detail::flat_tree<unsigned int,boost::container::container_detail::pair<unsigned int,unsigned char>,boost::container::container_detail::select1st<boost::container::container_detail::pair<unsigned int,unsigned char> >,std::less<unsigned int>,std::allocator<boost::container::container_detail::pair<unsigned int,unsigned char> > >::Data : boost::container::container_detail::flat_tree_value_compare<std::less<unsigned int>,boost::container::container_detail::pair<unsigned int,unsigned char>,boost::container::container_detail::select1st<boost::container::container_detail::pair<unsigned int,unsigned char> > >",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "container",
            "container_detail",
            "flat_tree<unsigned int,boost::container::container_detail::pair<unsigned int,unsigned char>,boost::container::container_detail::select1st<boost::container::container_detail::pair<unsigned int,unsigned char> >,std::less<unsigned int>,std::allocator<boost::container::container_detail::pair<unsigned int,unsigned char> > >",
            "Data"
        ]
    );
}

#[test]
fn test_type_117() {
    let result = parse_type(
        "struct __cppobj boost::container::container_detail::insert_move_proxy<boost::container::container_detail::static_storage_allocator<SUpgradeInfo const *,4>,SUpgradeInfo const * *>",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "container",
            "container_detail",
            "insert_move_proxy<boost::container::container_detail::static_storage_allocator<SUpgradeInfo const *,4>,SUpgradeInfo const * *>"
        ]
    );
}

#[test]
fn test_type_118() {
    let result = parse_type(
        "struct __cppobj boost::container::container_detail::pair<CHashString,boost::container::static_vector<std::pair<PlayerId_Steam,int>,5> >",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "container",
            "container_detail",
            "pair<CHashString,boost::container::static_vector<std::pair<PlayerId_Steam,int>,5> >"
        ]
    );
}

#[test]
fn test_type_119() {
    let result = parse_type(
        "struct __cppobj boost::container::container_detail::scoped_destructor_n<std::allocator<boost::container::container_detail::pair<CHashString,boost::variant<boost::detail::variant::over_sequence<boost::mpl::vector<unsigned __int64,float,CMatrix4f,boost::mpl::na,boost::mpl::na,boost::mpl::na,boost::mpl::na,boost::mpl::na,boost::mpl::na,boost::mpl::na,boost::mpl::na,boost::mpl::na,boost::mpl::na,boost::mpl::na,boost::mpl::na,boost::mpl::na,boost::mpl::na,boost::mpl::na,boost::mpl::na,boost::mpl::na> >> > > >",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "container",
            "container_detail",
            "scoped_destructor_n<std::allocator<boost::container::container_detail::pair<CHashString,boost::variant<boost::detail::variant::over_sequence<boost::mpl::vector<unsigned __int64,float,CMatrix4f,boost::mpl::na,boost::mpl::na,boost::mpl::na,boost::mpl::na,boost::mpl::na,boost::mpl::na,boost::mpl::na,boost::mpl::na,boost::mpl::na,boost::mpl::na,boost::mpl::na,boost::mpl::na,boost::mpl::na,boost::mpl::na,boost::mpl::na,boost::mpl::na,boost::mpl::na> >> > > >"
        ]
    );
}

#[test]
fn test_type_120() {
    let result = parse_type(
        "struct __cppobj boost::container::container_detail::select1st<boost::container::container_detail::pair<ava::idstring<PropertyContainerIdStringTag,1>,TVariant<std::allocator<int> > > >",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "container",
            "container_detail",
            "select1st<boost::container::container_detail::pair<ava::idstring<PropertyContainerIdStringTag,1>,TVariant<std::allocator<int> > > >"
        ]
    );
}

#[test]
fn test_type_121() {
    let result = parse_type(
        "struct __cppobj boost::container::container_detail::value_init<std::vector<boost::shared_ptr<CSessionInfo>> *>",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "container",
            "container_detail",
            "value_init<std::vector<boost::shared_ptr<CSessionInfo>> *>"
        ]
    );
}

#[test]
fn test_type_122() {
    let result = parse_type(
        "struct __cppobj boost::container::container_detail::vec_iterator<boost::container::container_detail::pair<CHashString,boost::container::static_vector<std::pair<PlayerId_Steam,int>,5> > *,1>",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "container",
            "container_detail",
            "vec_iterator<boost::container::container_detail::pair<CHashString,boost::container::static_vector<std::pair<PlayerId_Steam,int>,5> > *,1>"
        ]
    );
}

#[test]
fn test_type_123() {
    let result = parse_type(
        "struct __cppobj boost::container::container_detail::vec_iterator<std::pair<CHashString,SBucketDefinition *> *,1>",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "container",
            "container_detail",
            "vec_iterator<std::pair<CHashString,SBucketDefinition *> *,1>"
        ]
    );
}

#[test]
fn test_type_124() {
    let result = parse_type(
        "struct __cppobj boost::container::container_detail::vec_iterator<unsigned int *,1>",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "container",
            "container_detail",
            "vec_iterator<unsigned int *,1>"
        ]
    );
}

#[test]
fn test_type_125() {
    let result = parse_type(
        "struct __cppobj boost::container::container_detail::vector_alloc_holder<CStatisticContext::allocator_t<std::pair<CHashString,CHashString> >,boost::container::container_detail::integral_constant<unsigned int,1> > : CStatisticContext::allocator_t<std::pair<CHashString,CHashString> >",
    );
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
fn test_type_126() {
    let result = parse_type(
        "struct __cppobj boost::container::container_detail::vector_value_traits<boost::container::container_detail::pair<CHashString,boost::container::static_vector<std::pair<PlayerId_Steam,int>,5> >,std::allocator<boost::container::container_detail::pair<CHashString,boost::container::static_vector<std::pair<PlayerId_Steam,int>,5> > > >",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "container",
            "container_detail",
            "vector_value_traits<boost::container::container_detail::pair<CHashString,boost::container::static_vector<std::pair<PlayerId_Steam,int>,5> >,std::allocator<boost::container::container_detail::pair<CHashString,boost::container::static_vector<std::pair<PlayerId_Steam,int>,5> > > >"
        ]
    );
}

#[test]
fn test_type_127() {
    let result = parse_type(
        "struct __cppobj boost::container::container_detail::version_type<boost::container::container_detail::static_storage_allocator<int,17>,0> : boost::container::container_detail::integral_constant<unsigned int,0>",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "container",
            "container_detail",
            "version_type<boost::container::container_detail::static_storage_allocator<int,17>,0>"
        ]
    );
}

#[test]
fn test_type_128() {
    let result = parse_type(
        "struct __cppobj boost::container::flat_map<CHashString,std::unique_ptr<CLinkBase_Steam>,std::less<CHashString>,std::allocator<std::pair<CHashString,std::unique_ptr<CLinkBase_Steam> > > >",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "container",
            "flat_map<CHashString,std::unique_ptr<CLinkBase_Steam>,std::less<CHashString>,std::allocator<std::pair<CHashString,std::unique_ptr<CLinkBase_Steam> > > >"
        ]
    );
}

#[test]
fn test_type_129() {
    let result = parse_type(
        "struct __cppobj boost::container::vector<boost::container::container_detail::pair<CHashString,boost::container::static_vector<std::pair<PlayerId_Steam,int>,5> >,std::allocator<boost::container::container_detail::pair<CHashString,boost::container::static_vector<std::pair<PlayerId_Steam,int>,5> > > >",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "container",
            "vector<boost::container::container_detail::pair<CHashString,boost::container::static_vector<std::pair<PlayerId_Steam,int>,5> >,std::allocator<boost::container::container_detail::pair<CHashString,boost::container::static_vector<std::pair<PlayerId_Steam,int>,5> > > >"
        ]
    );
}

#[test]
fn test_type_130() {
    let result = parse_type(
        "struct __cppobj boost::container::vector<boost::container::container_detail::pair<CHashString,std::pair<CStatisticDelegateHolder<TDelegate<void __cdecl(CStatistic<__int64> const *,__int64,__int64,CStatisticContext const &)> >,int> >,std::allocator<boost::container::container_detail::pair<CHashString,std::pair<CStatisticDelegateHolder<TDelegate<void __cdecl(CStatistic<__int64> const *,__int64,__int64,CStatisticContext const &)> >,int> > > >",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "container",
            "vector<boost::container::container_detail::pair<CHashString,std::pair<CStatisticDelegateHolder<TDelegate<void __cdecl(CStatistic<__int64> const *,__int64,__int64,CStatisticContext const &)> >,int> >,std::allocator<boost::container::container_detail::pair<CHashString,std::pair<CStatisticDelegateHolder<TDelegate<void __cdecl(CStatistic<__int64> const *,__int64,__int64,CStatisticContext const &)> >,int> > > >"
        ]
    );
}

#[test]
fn test_type_131() {
    let result = parse_type("struct __cppobj boost::detail::addr_impl_ref<<lambda133> >");
    assert_eq!(
        result,
        vec!["boost", "detail", "addr_impl_ref<<lambda133> >"]
    );
}

#[test]
fn test_type_132() {
    let result = parse_type("struct __cppobj boost::detail::addr_impl_ref<<lambda173> >");
    assert_eq!(
        result,
        vec!["boost", "detail", "addr_impl_ref<<lambda173> >"]
    );
}

#[test]
fn test_type_133() {
    let result = parse_type("struct __cppobj boost::detail::addr_impl_ref<<lambda316> >");
    assert_eq!(
        result,
        vec!["boost", "detail", "addr_impl_ref<<lambda316> >"]
    );
}

#[test]
fn test_type_134() {
    let result = parse_type("struct __cppobj boost::detail::addr_impl_ref<<lambda343> >");
    assert_eq!(
        result,
        vec!["boost", "detail", "addr_impl_ref<<lambda343> >"]
    );
}

#[test]
fn test_type_135() {
    let result = parse_type("struct __cppobj boost::detail::addr_impl_ref<float>");
    assert_eq!(result, vec!["boost", "detail", "addr_impl_ref<float>"]);
}

#[test]
fn test_type_136() {
    let result = parse_type("struct __cppobj boost::detail::addressof_impl<<lambda165> >");
    assert_eq!(
        result,
        vec!["boost", "detail", "addressof_impl<<lambda165> >"]
    );
}

#[test]
fn test_type_137() {
    let result = parse_type("struct __cppobj boost::detail::addressof_impl<<lambda206> >");
    assert_eq!(
        result,
        vec!["boost", "detail", "addressof_impl<<lambda206> >"]
    );
}

#[test]
fn test_type_138() {
    let result = parse_type("struct __cppobj boost::detail::addressof_impl<<lambda221> >");
    assert_eq!(
        result,
        vec!["boost", "detail", "addressof_impl<<lambda221> >"]
    );
}

#[test]
fn test_type_139() {
    let result = parse_type("struct __cppobj boost::detail::addressof_impl<<lambda335> >");
    assert_eq!(
        result,
        vec!["boost", "detail", "addressof_impl<<lambda335> >"]
    );
}

#[test]
fn test_type_140() {
    let result = parse_type("struct __cppobj boost::detail::core_typeid_<<lambda160> >");
    assert_eq!(
        result,
        vec!["boost", "detail", "core_typeid_<<lambda160> >"]
    );
}

#[test]
fn test_type_141() {
    let result = parse_type("struct __cppobj boost::detail::core_typeid_<<lambda243> >");
    assert_eq!(
        result,
        vec!["boost", "detail", "core_typeid_<<lambda243> >"]
    );
}

#[test]
fn test_type_142() {
    let result = parse_type("struct __cppobj boost::detail::core_typeid_<<lambda326> >");
    assert_eq!(
        result,
        vec!["boost", "detail", "core_typeid_<<lambda326> >"]
    );
}

#[test]
fn test_type_143() {
    let result = parse_type(
        "struct __cppobj boost::detail::core_typeid_<boost::spirit::qi::detail::parser_binder<boost::spirit::qi::alternative<boost::fusion::cons<boost::spirit::qi::any_real_parser<float,boost::spirit::qi::real_policies<float> >,boost::fusion::cons<boost::spirit::qi::reference<boost::spirit::qi::rule<char const *,dynamo::ast::intrinsic_op __cdecl(void),boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::char_code<boost::spirit::tag::space,boost::spirit::char_encoding::ascii> >,0>,boost::spirit::unused_type,boost::spirit::unused_type> const >,boost::fusion::cons<boost::spirit::qi::reference<boost::spirit::qi::rule<char const *,dynamo::ast::identifier __cdecl(void),boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::char_code<boost::spirit::tag::space,boost::spirit::char_encoding::ascii> >,0>,boost::spirit::unused_type,boost::spirit::unused_type> const >,boost::fusion::cons<boost::spirit::qi::expect<boost::fusion::cons<boost::spirit::qi::literal_char<boost::spirit::char_encoding::standard,1,0>,boost::fusion::cons<boost::spirit::qi::reference<boost::spirit::qi::rule<char const *,dynamo::ast::expression __cdecl(void),boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::char_code<boost::spirit::tag::space,boost::spirit::char_encoding::ascii> >,0>,boost::spirit::unused_type,boost::spirit::unused_type> const >,boost::fusion::cons<boost::spirit::qi::literal_char<boost::spirit::char_encoding::standard,1,0>,boost::fusion::nil_> > > >,boost::fusion::nil_> > > > >,boost::mpl::bool_<0> > >",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "detail",
            "core_typeid_<boost::spirit::qi::detail::parser_binder<boost::spirit::qi::alternative<boost::fusion::cons<boost::spirit::qi::any_real_parser<float,boost::spirit::qi::real_policies<float> >,boost::fusion::cons<boost::spirit::qi::reference<boost::spirit::qi::rule<char const *,dynamo::ast::intrinsic_op __cdecl(void),boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::char_code<boost::spirit::tag::space,boost::spirit::char_encoding::ascii> >,0>,boost::spirit::unused_type,boost::spirit::unused_type> const >,boost::fusion::cons<boost::spirit::qi::reference<boost::spirit::qi::rule<char const *,dynamo::ast::identifier __cdecl(void),boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::char_code<boost::spirit::tag::space,boost::spirit::char_encoding::ascii> >,0>,boost::spirit::unused_type,boost::spirit::unused_type> const >,boost::fusion::cons<boost::spirit::qi::expect<boost::fusion::cons<boost::spirit::qi::literal_char<boost::spirit::char_encoding::standard,1,0>,boost::fusion::cons<boost::spirit::qi::reference<boost::spirit::qi::rule<char const *,dynamo::ast::expression __cdecl(void),boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::char_code<boost::spirit::tag::space,boost::spirit::char_encoding::ascii> >,0>,boost::spirit::unused_type,boost::spirit::unused_type> const >,boost::fusion::cons<boost::spirit::qi::literal_char<boost::spirit::char_encoding::standard,1,0>,boost::fusion::nil_> > > >,boost::fusion::nil_> > > > >,boost::mpl::bool_<0> > >"
        ]
    );
}

#[test]
fn test_type_144() {
    let result = parse_type(
        "struct __cppobj boost::detail::core_typeid_<CGameObjectCreator<CRagdollObject>::SGameObjectDestroyer>",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "detail",
            "core_typeid_<CGameObjectCreator<CRagdollObject>::SGameObjectDestroyer>"
        ]
    );
}

#[test]
fn test_type_145() {
    let result = parse_type(
        "struct __cppobj boost::detail::function::function_obj_invoker0<<lambda181>,float>",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "detail",
            "function",
            "function_obj_invoker0<<lambda181>,float>"
        ]
    );
}

#[test]
fn test_type_146() {
    let result = parse_type(
        "struct __cppobj boost::detail::function::function_obj_invoker0<<lambda184>,float>",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "detail",
            "function",
            "function_obj_invoker0<<lambda184>,float>"
        ]
    );
}

#[test]
fn test_type_147() {
    let result = parse_type(
        "struct __cppobj boost::detail::function::function_obj_invoker0<<lambda351>,float>",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "detail",
            "function",
            "function_obj_invoker0<<lambda351>,float>"
        ]
    );
}

#[test]
fn test_type_148() {
    let result =
        parse_type("struct __cppobj boost::detail::function::functor_manager_common<<lambda149> >");
    assert_eq!(
        result,
        vec![
            "boost",
            "detail",
            "function",
            "functor_manager_common<<lambda149> >"
        ]
    );
}

#[test]
fn test_type_149() {
    let result =
        parse_type("struct __cppobj boost::detail::function::functor_manager_common<<lambda225> >");
    assert_eq!(
        result,
        vec![
            "boost",
            "detail",
            "function",
            "functor_manager_common<<lambda225> >"
        ]
    );
}

#[test]
fn test_type_150() {
    let result = parse_type(
        "struct __cppobj boost::detail::function::functor_manager_common<boost::spirit::qi::detail::parser_binder<boost::spirit::qi::expect<boost::fusion::cons<boost::spirit::qi::sequence<boost::fusion::cons<boost::spirit::qi::reference<boost::spirit::qi::rule<char const *,dynamo::ast::identifier __cdecl(void),boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::char_code<boost::spirit::tag::space,boost::spirit::char_encoding::ascii> >,0>,boost::spirit::unused_type,boost::spirit::unused_type> const >,boost::fusion::cons<boost::spirit::qi::literal_char<boost::spirit::char_encoding::standard,1,0>,boost::fusion::nil_> > >,boost::fusion::cons<boost::spirit::qi::reference<boost::spirit::qi::rule<char const *,dynamo::ast::expression __cdecl(void),boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::char_code<boost::spirit::tag::space,boost::spirit::char_encoding::ascii> >,0>,boost::spirit::unused_type,boost::spirit::unused_type> const >,boost::fusion::nil_> > >,boost::mpl::bool_<0> > >",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "detail",
            "function",
            "functor_manager_common<boost::spirit::qi::detail::parser_binder<boost::spirit::qi::expect<boost::fusion::cons<boost::spirit::qi::sequence<boost::fusion::cons<boost::spirit::qi::reference<boost::spirit::qi::rule<char const *,dynamo::ast::identifier __cdecl(void),boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::char_code<boost::spirit::tag::space,boost::spirit::char_encoding::ascii> >,0>,boost::spirit::unused_type,boost::spirit::unused_type> const >,boost::fusion::cons<boost::spirit::qi::literal_char<boost::spirit::char_encoding::standard,1,0>,boost::fusion::nil_> > >,boost::fusion::cons<boost::spirit::qi::reference<boost::spirit::qi::rule<char const *,dynamo::ast::expression __cdecl(void),boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::char_code<boost::spirit::tag::space,boost::spirit::char_encoding::ascii> >,0>,boost::spirit::unused_type,boost::spirit::unused_type> const >,boost::fusion::nil_> > >,boost::mpl::bool_<0> > >"
        ]
    );
}

#[test]
fn test_type_151() {
    let result =
        parse_type("struct __cppobj boost::detail::function::functor_manager<<lambda272> >");
    assert_eq!(
        result,
        vec![
            "boost",
            "detail",
            "function",
            "functor_manager<<lambda272> >"
        ]
    );
}

#[test]
fn test_type_152() {
    let result =
        parse_type("struct __cppobj boost::detail::function::functor_manager<<lambda285> >");
    assert_eq!(
        result,
        vec![
            "boost",
            "detail",
            "function",
            "functor_manager<<lambda285> >"
        ]
    );
}

#[test]
fn test_type_153() {
    let result =
        parse_type("struct __cppobj boost::detail::function::functor_manager<<lambda303> >");
    assert_eq!(
        result,
        vec![
            "boost",
            "detail",
            "function",
            "functor_manager<<lambda303> >"
        ]
    );
}

#[test]
fn test_type_154() {
    let result =
        parse_type("struct __cppobj boost::detail::function::functor_manager<<lambda368> >");
    assert_eq!(
        result,
        vec![
            "boost",
            "detail",
            "function",
            "functor_manager<<lambda368> >"
        ]
    );
}

#[test]
fn test_type_155() {
    let result =
        parse_type("struct __cppobj boost::detail::function::functor_manager<<lambda389> >");
    assert_eq!(
        result,
        vec![
            "boost",
            "detail",
            "function",
            "functor_manager<<lambda389> >"
        ]
    );
}

#[test]
fn test_type_156() {
    let result = parse_type("struct __cppobj boost::detail::sp_aligned_storage<24,8>");
    assert_eq!(result, vec!["boost", "detail", "sp_aligned_storage<24,8>"]);
}

#[test]
fn test_type_157() {
    let result = parse_type(
        "struct __cppobj boost::detail::sp_counted_impl_pd<TaskQueue::detail::CSyncWork<A0x743c4eea::<lambda205>,void,A0x743c4eea::SJobResult> *,boost::detail::sp_ms_deleter<TaskQueue::detail::CSyncWork<A0x743c4eea::<lambda205>,void,A0x743c4eea::SJobResult> > > : boost::detail::sp_counted_base",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "detail",
            "sp_counted_impl_pd<TaskQueue::detail::CSyncWork<A0x743c4eea::<lambda205>,void,A0x743c4eea::SJobResult> *,boost::detail::sp_ms_deleter<TaskQueue::detail::CSyncWork<A0x743c4eea::<lambda205>,void,A0x743c4eea::SJobResult> > >"
        ]
    );
}

#[test]
fn test_type_158() {
    let result = parse_type("struct __cppobj boost::detail::sp_empty");
    assert_eq!(result, vec!["boost", "detail", "sp_empty"]);
}

#[test]
fn test_type_159() {
    let result = parse_type("struct __cppobj boost::detail::sp_ms_deleter<CFriend>");
    assert_eq!(result, vec!["boost", "detail", "sp_ms_deleter<CFriend>"]);
}

#[test]
fn test_type_160() {
    let result = parse_type(
        "struct __cppobj boost::detail::sp_ms_deleter<TaskQueue::detail::CSyncWork<<lambda175>,void,enum NOnline::EErrorCode> >",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "detail",
            "sp_ms_deleter<TaskQueue::detail::CSyncWork<<lambda175>,void,enum NOnline::EErrorCode> >"
        ]
    );
}

#[test]
fn test_type_161() {
    let result = parse_type(
        "struct __cppobj boost::detail::sp_ms_deleter<TaskQueue::detail::CSyncWork<A0x743c4eea::<lambda213>,void,A0x743c4eea::SJobResult> >",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "detail",
            "sp_ms_deleter<TaskQueue::detail::CSyncWork<A0x743c4eea::<lambda213>,void,A0x743c4eea::SJobResult> >"
        ]
    );
}

#[test]
fn test_type_162() {
    let result = parse_type(
        "struct __cppobj boost::detail::variant::make_initializer_node::apply<boost::mpl::pair<boost::detail::variant::make_initializer_node::apply<boost::mpl::pair<boost::detail::variant::make_initializer_node::apply<boost::mpl::pair<boost::detail::variant::make_initializer_node::apply<boost::mpl::pair<boost::detail::variant::initializer_root,boost::mpl::int_<0> >,boost::mpl::l_iter<boost::mpl::list7<dynamo::ast::nil,float,dynamo::ast::identifier,boost::recursive_wrapper<dynamo::ast::unary_op>,boost::recursive_wrapper<dynamo::ast::binary_op>,boost::recursive_wrapper<dynamo::ast::intrinsic_op>,boost::recursive_wrapper<dynamo::ast::expression> > > >::initializer_node,boost::mpl::int_<1> >,boost::mpl::l_iter<boost::mpl::list6<float,dynamo::ast::identifier,boost::recursive_wrapper<dynamo::ast::unary_op>,boost::recursive_wrapper<dynamo::ast::binary_op>,boost::recursive_wrapper<dynamo::ast::intrinsic_op>,boost::recursive_wrapper<dynamo::ast::expression> > > >::initializer_node,boost::mpl::int_<2> >,boost::mpl::l_iter<boost::mpl::list5<dynamo::ast::identifier,boost::recursive_wrapper<dynamo::ast::unary_op>,boost::recursive_wrapper<dynamo::ast::binary_op>,boost::recursive_wrapper<dynamo::ast::intrinsic_op>,boost::recursive_wrapper<dynamo::ast::expression> > > >::initializer_node,boost::mpl::int_<3> >,boost::mpl::l_iter<boost::mpl::list4<boost::recursive_wrapper<dynamo::ast::unary_op>,boost::recursive_wrapper<dynamo::ast::binary_op>,boost::recursive_wrapper<dynamo::ast::intrinsic_op>,boost::recursive_wrapper<dynamo::ast::expression> > > >::initializer_node : boost::detail::variant::make_initializer_node::apply<boost::mpl::pair<boost::detail::variant::make_initializer_node::apply<boost::mpl::pair<boost::detail::variant::make_initializer_node::apply<boost::mpl::pair<boost::detail::variant::initializer_root,boost::mpl::int_<0> >,boost::mpl::l_iter<boost::mpl::list7<dynamo::ast::nil,float,dynamo::ast::identifier,boost::recursive_wrapper<dynamo::ast::unary_op>,boost::recursive_wrapper<dynamo::ast::binary_op>,boost::recursive_wrapper<dynamo::ast::intrinsic_op>,boost::recursive_wrapper<dynamo::ast::expression> > > >::initializer_node,boost::mpl::int_<1> >,boost::mpl::l_iter<boost::mpl::list6<float,dynamo::ast::identifier,boost::recursive_wrapper<dynamo::ast::unary_op>,boost::recursive_wrapper<dynamo::ast::binary_op>,boost::recursive_wrapper<dynamo::ast::intrinsic_op>,boost::recursive_wrapper<dynamo::ast::expression> > > >::initializer_node,boost::mpl::int_<2> >,boost::mpl::l_iter<boost::mpl::list5<dynamo::ast::identifier,boost::recursive_wrapper<dynamo::ast::unary_op>,boost::recursive_wrapper<dynamo::ast::binary_op>,boost::recursive_wrapper<dynamo::ast::intrinsic_op>,boost::recursive_wrapper<dynamo::ast::expression> > > >::initializer_node",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "detail",
            "variant",
            "make_initializer_node",
            "apply<boost::mpl::pair<boost::detail::variant::make_initializer_node::apply<boost::mpl::pair<boost::detail::variant::make_initializer_node::apply<boost::mpl::pair<boost::detail::variant::make_initializer_node::apply<boost::mpl::pair<boost::detail::variant::initializer_root,boost::mpl::int_<0> >,boost::mpl::l_iter<boost::mpl::list7<dynamo::ast::nil,float,dynamo::ast::identifier,boost::recursive_wrapper<dynamo::ast::unary_op>,boost::recursive_wrapper<dynamo::ast::binary_op>,boost::recursive_wrapper<dynamo::ast::intrinsic_op>,boost::recursive_wrapper<dynamo::ast::expression> > > >::initializer_node,boost::mpl::int_<1> >,boost::mpl::l_iter<boost::mpl::list6<float,dynamo::ast::identifier,boost::recursive_wrapper<dynamo::ast::unary_op>,boost::recursive_wrapper<dynamo::ast::binary_op>,boost::recursive_wrapper<dynamo::ast::intrinsic_op>,boost::recursive_wrapper<dynamo::ast::expression> > > >::initializer_node,boost::mpl::int_<2> >,boost::mpl::l_iter<boost::mpl::list5<dynamo::ast::identifier,boost::recursive_wrapper<dynamo::ast::unary_op>,boost::recursive_wrapper<dynamo::ast::binary_op>,boost::recursive_wrapper<dynamo::ast::intrinsic_op>,boost::recursive_wrapper<dynamo::ast::expression> > > >::initializer_node,boost::mpl::int_<3> >,boost::mpl::l_iter<boost::mpl::list4<boost::recursive_wrapper<dynamo::ast::unary_op>,boost::recursive_wrapper<dynamo::ast::binary_op>,boost::recursive_wrapper<dynamo::ast::intrinsic_op>,boost::recursive_wrapper<dynamo::ast::expression> > > >",
            "initializer_node"
        ]
    );
}

#[test]
fn test_type_163() {
    let result = parse_type(
        "struct __cppobj boost::detail::variant::make_initializer_node::apply<boost::mpl::pair<boost::detail::variant::make_initializer_node::apply<boost::mpl::pair<boost::detail::variant::make_initializer_node::apply<boost::mpl::pair<boost::detail::variant::make_initializer_node::apply<boost::mpl::pair<boost::detail::variant::make_initializer_node::apply<boost::mpl::pair<boost::detail::variant::make_initializer_node::apply<boost::mpl::pair<boost::detail::variant::make_initializer_node::apply<boost::mpl::pair<boost::detail::variant::initializer_root,boost::mpl::int_<0> >,boost::mpl::l_iter<boost::mpl::list7<dynamo::ast::nil,float,dynamo::ast::identifier,boost::recursive_wrapper<dynamo::ast::unary_op>,boost::recursive_wrapper<dynamo::ast::binary_op>,boost::recursive_wrapper<dynamo::ast::intrinsic_op>,boost::recursive_wrapper<dynamo::ast::expression> > > >::initializer_node,boost::mpl::int_<1> >,boost::mpl::l_iter<boost::mpl::list6<float,dynamo::ast::identifier,boost::recursive_wrapper<dynamo::ast::unary_op>,boost::recursive_wrapper<dynamo::ast::binary_op>,boost::recursive_wrapper<dynamo::ast::intrinsic_op>,boost::recursive_wrapper<dynamo::ast::expression> > > >::initializer_node,boost::mpl::int_<2> >,boost::mpl::l_iter<boost::mpl::list5<dynamo::ast::identifier,boost::recursive_wrapper<dynamo::ast::unary_op>,boost::recursive_wrapper<dynamo::ast::binary_op>,boost::recursive_wrapper<dynamo::ast::intrinsic_op>,boost::recursive_wrapper<dynamo::ast::expression> > > >::initializer_node,boost::mpl::int_<3> >,boost::mpl::l_iter<boost::mpl::list4<boost::recursive_wrapper<dynamo::ast::unary_op>,boost::recursive_wrapper<dynamo::ast::binary_op>,boost::recursive_wrapper<dynamo::ast::intrinsic_op>,boost::recursive_wrapper<dynamo::ast::expression> > > >::initializer_node,boost::mpl::int_<4> >,boost::mpl::l_iter<boost::mpl::list3<boost::recursive_wrapper<dynamo::ast::binary_op>,boost::recursive_wrapper<dynamo::ast::intrinsic_op>,boost::recursive_wrapper<dynamo::ast::expression> > > >::initializer_node,boost::mpl::int_<5> >,boost::mpl::l_iter<boost::mpl::list2<boost::recursive_wrapper<dynamo::ast::intrinsic_op>,boost::recursive_wrapper<dynamo::ast::expression> > > >::initializer_node,boost::mpl::int_<6> >,boost::mpl::l_iter<boost::mpl::list1<boost::recursive_wrapper<dynamo::ast::expression> > > >::initializer_node : boost::detail::variant::make_initializer_node::apply<boost::mpl::pair<boost::detail::variant::make_initializer_node::apply<boost::mpl::pair<boost::detail::variant::make_initializer_node::apply<boost::mpl::pair<boost::detail::variant::make_initializer_node::apply<boost::mpl::pair<boost::detail::variant::make_initializer_node::apply<boost::mpl::pair<boost::detail::variant::make_initializer_node::apply<boost::mpl::pair<boost::detail::variant::initializer_root,boost::mpl::int_<0> >,boost::mpl::l_iter<boost::mpl::list7<dynamo::ast::nil,float,dynamo::ast::identifier,boost::recursive_wrapper<dynamo::ast::unary_op>,boost::recursive_wrapper<dynamo::ast::binary_op>,boost::recursive_wrapper<dynamo::ast::intrinsic_op>,boost::recursive_wrapper<dynamo::ast::expression> > > >::initializer_node,boost::mpl::int_<1> >,boost::mpl::l_iter<boost::mpl::list6<float,dynamo::ast::identifier,boost::recursive_wrapper<dynamo::ast::unary_op>,boost::recursive_wrapper<dynamo::ast::binary_op>,boost::recursive_wrapper<dynamo::ast::intrinsic_op>,boost::recursive_wrapper<dynamo::ast::expression> > > >::initializer_node,boost::mpl::int_<2> >,boost::mpl::l_iter<boost::mpl::list5<dynamo::ast::identifier,boost::recursive_wrapper<dynamo::ast::unary_op>,boost::recursive_wrapper<dynamo::ast::binary_op>,boost::recursive_wrapper<dynamo::ast::intrinsic_op>,boost::recursive_wrapper<dynamo::ast::expression> > > >::initializer_node,boost::mpl::int_<3> >,boost::mpl::l_iter<boost::mpl::list4<boost::recursive_wrapper<dynamo::ast::unary_op>,boost::recursive_wrapper<dynamo::ast::binary_op>,boost::recursive_wrapper<dynamo::ast::intrinsic_op>,boost::recursive_wrapper<dynamo::ast::expression> > > >::initializer_node,boost::mpl::int_<4> >,boost::mpl::l_iter<boost::mpl::list3<boost::recursive_wrapper<dynamo::ast::binary_op>,boost::recursive_wrapper<dynamo::ast::intrinsic_op>,boost::recursive_wrapper<dynamo::ast::expression> > > >::initializer_node,boost::mpl::int_<5> >,boost::mpl::l_iter<boost::mpl::list2<boost::recursive_wrapper<dynamo::ast::intrinsic_op>,boost::recursive_wrapper<dynamo::ast::expression> > > >::initializer_node",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "detail",
            "variant",
            "make_initializer_node",
            "apply<boost::mpl::pair<boost::detail::variant::make_initializer_node::apply<boost::mpl::pair<boost::detail::variant::make_initializer_node::apply<boost::mpl::pair<boost::detail::variant::make_initializer_node::apply<boost::mpl::pair<boost::detail::variant::make_initializer_node::apply<boost::mpl::pair<boost::detail::variant::make_initializer_node::apply<boost::mpl::pair<boost::detail::variant::make_initializer_node::apply<boost::mpl::pair<boost::detail::variant::initializer_root,boost::mpl::int_<0> >,boost::mpl::l_iter<boost::mpl::list7<dynamo::ast::nil,float,dynamo::ast::identifier,boost::recursive_wrapper<dynamo::ast::unary_op>,boost::recursive_wrapper<dynamo::ast::binary_op>,boost::recursive_wrapper<dynamo::ast::intrinsic_op>,boost::recursive_wrapper<dynamo::ast::expression> > > >::initializer_node,boost::mpl::int_<1> >,boost::mpl::l_iter<boost::mpl::list6<float,dynamo::ast::identifier,boost::recursive_wrapper<dynamo::ast::unary_op>,boost::recursive_wrapper<dynamo::ast::binary_op>,boost::recursive_wrapper<dynamo::ast::intrinsic_op>,boost::recursive_wrapper<dynamo::ast::expression> > > >::initializer_node,boost::mpl::int_<2> >,boost::mpl::l_iter<boost::mpl::list5<dynamo::ast::identifier,boost::recursive_wrapper<dynamo::ast::unary_op>,boost::recursive_wrapper<dynamo::ast::binary_op>,boost::recursive_wrapper<dynamo::ast::intrinsic_op>,boost::recursive_wrapper<dynamo::ast::expression> > > >::initializer_node,boost::mpl::int_<3> >,boost::mpl::l_iter<boost::mpl::list4<boost::recursive_wrapper<dynamo::ast::unary_op>,boost::recursive_wrapper<dynamo::ast::binary_op>,boost::recursive_wrapper<dynamo::ast::intrinsic_op>,boost::recursive_wrapper<dynamo::ast::expression> > > >::initializer_node,boost::mpl::int_<4> >,boost::mpl::l_iter<boost::mpl::list3<boost::recursive_wrapper<dynamo::ast::binary_op>,boost::recursive_wrapper<dynamo::ast::intrinsic_op>,boost::recursive_wrapper<dynamo::ast::expression> > > >::initializer_node,boost::mpl::int_<5> >,boost::mpl::l_iter<boost::mpl::list2<boost::recursive_wrapper<dynamo::ast::intrinsic_op>,boost::recursive_wrapper<dynamo::ast::expression> > > >::initializer_node,boost::mpl::int_<6> >,boost::mpl::l_iter<boost::mpl::list1<boost::recursive_wrapper<dynamo::ast::expression> > > >",
            "initializer_node"
        ]
    );
}

#[test]
fn test_type_164() {
    let result = parse_type(
        "struct __cppobj boost::detail::variant::visitation_impl_step<boost::mpl::v_iter<boost::mpl::vector14<bool,int,float,std::string,char,unsigned char,signed char,unsigned int,short,unsigned short,long,unsigned long,__int64,unsigned __int64>,10>,boost::mpl::v_iter<boost::mpl::vector14<bool,int,float,std::string,char,unsigned char,signed char,unsigned int,short,unsigned short,long,unsigned long,__int64,unsigned __int64>,14> >",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "detail",
            "variant",
            "visitation_impl_step<boost::mpl::v_iter<boost::mpl::vector14<bool,int,float,std::string,char,unsigned char,signed char,unsigned int,short,unsigned short,long,unsigned long,__int64,unsigned __int64>,10>,boost::mpl::v_iter<boost::mpl::vector14<bool,int,float,std::string,char,unsigned char,signed char,unsigned int,short,unsigned short,long,unsigned long,__int64,unsigned __int64>,14> >"
        ]
    );
}

#[test]
fn test_type_165() {
    let result = parse_type(
        "struct __cppobj boost::equality_comparable1<boost::gregorian::date,boost::detail::empty_base<boost::gregorian::date> > : boost::detail::empty_base<boost::gregorian::date>",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "equality_comparable1<boost::gregorian::date,boost::detail::empty_base<boost::gregorian::date> >"
        ]
    );
}

#[test]
fn test_type_166() {
    let result = parse_type(
        "struct __cppobj boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_get> >::clone_tag",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "exception_detail",
            "clone_impl<boost::exception_detail::error_info_injector<boost::bad_get> >",
            "clone_tag"
        ]
    );
}

#[test]
fn test_type_167() {
    let result = parse_type(
        "struct __cppobj boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_weak_ptr> >::clone_tag",
    );
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
fn test_type_168() {
    let result = parse_type(
        "struct __cppobj boost::function<bool __cdecl(char const * &,char const * const &,boost::spirit::context<boost::fusion::cons<dynamo::ast::binary_op &,boost::fusion::nil_>,boost::fusion::vector0<void> > &,boost::spirit::qi::char_class<boost::spirit::tag::char_code<boost::spirit::tag::space,boost::spirit::char_encoding::ascii> > const &)> : boost::function4<bool,char const * &,char const * const &,boost::spirit::context<boost::fusion::cons<dynamo::ast::binary_op &,boost::fusion::nil_>,boost::fusion::vector0<void> > &,boost::spirit::qi::char_class<boost::spirit::tag::char_code<boost::spirit::tag::space,boost::spirit::char_encoding::ascii> > const &>",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "function<bool __cdecl(char const * &,char const * const &,boost::spirit::context<boost::fusion::cons<dynamo::ast::binary_op &,boost::fusion::nil_>,boost::fusion::vector0<void> > &,boost::spirit::qi::char_class<boost::spirit::tag::char_code<boost::spirit::tag::space,boost::spirit::char_encoding::ascii> > const &)>"
        ]
    );
}

#[test]
fn test_type_169() {
    let result = parse_type(
        "struct __cppobj boost::fusion::cons<boost::spirit::tag::char_code<boost::spirit::tag::space,boost::spirit::char_encoding::ascii>,boost::fusion::nil_> : boost::fusion::sequence_base<boost::fusion::cons<boost::spirit::tag::char_code<boost::spirit::tag::space,boost::spirit::char_encoding::ascii>,boost::fusion::nil_> >",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "fusion",
            "cons<boost::spirit::tag::char_code<boost::spirit::tag::space,boost::spirit::char_encoding::ascii>,boost::fusion::nil_>"
        ]
    );
}

#[test]
fn test_type_170() {
    let result = parse_type(
        "struct __cppobj boost::fusion::extension::access::struct_member<boost::phoenix::vector2<boost::phoenix::vector3<boost::phoenix::actor<boost::proto::exprns_::basic_expr<boost::phoenix::detail::tag::function_eval,boost::proto::argsns_::list3<boost::proto::exprns_::basic_expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<dynamo::annotation<char const *> >,0>,boost::phoenix::actor<boost::spirit::attribute<0> >,boost::phoenix::actor<boost::spirit::argument<0> > >,3> > const *,boost::fusion::vector<char const * &,char const * const &,char const * const &,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_> &,boost::spirit::context<boost::fusion::cons<dynamo::ast::assignment &,boost::fusion::nil_>,boost::fusion::vector0<void> > const &> &,boost::phoenix::default_actions const &>,1>",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "fusion",
            "extension",
            "access",
            "struct_member<boost::phoenix::vector2<boost::phoenix::vector3<boost::phoenix::actor<boost::proto::exprns_::basic_expr<boost::phoenix::detail::tag::function_eval,boost::proto::argsns_::list3<boost::proto::exprns_::basic_expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<dynamo::annotation<char const *> >,0>,boost::phoenix::actor<boost::spirit::attribute<0> >,boost::phoenix::actor<boost::spirit::argument<0> > >,3> > const *,boost::fusion::vector<char const * &,char const * const &,char const * const &,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_> &,boost::spirit::context<boost::fusion::cons<dynamo::ast::assignment &,boost::fusion::nil_>,boost::fusion::vector0<void> > const &> &,boost::phoenix::default_actions const &>,1>"
        ]
    );
}

#[test]
fn test_type_171() {
    let result = parse_type(
        "struct __cppobj boost::fusion::extension::access::struct_member<dynamo::ast::intrinsic_op,0>",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "fusion",
            "extension",
            "access",
            "struct_member<dynamo::ast::intrinsic_op,0>"
        ]
    );
}

#[test]
fn test_type_172() {
    let result = parse_type(
        "struct __cppobj boost::fusion::extension::begin_impl<boost::fusion::cons_tag>::apply<boost::fusion::cons<boost::spirit::qi::any_real_parser<float,boost::spirit::qi::real_policies<float> >,boost::fusion::cons<boost::spirit::qi::reference<boost::spirit::qi::rule<char const *,dynamo::ast::intrinsic_op __cdecl(void),boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::char_code<boost::spirit::tag::space,boost::spirit::char_encoding::ascii> >,0>,boost::spirit::unused_type,boost::spirit::unused_type> const >,boost::fusion::cons<boost::spirit::qi::reference<boost::spirit::qi::rule<char const *,dynamo::ast::identifier __cdecl(void),boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::char_code<boost::spirit::tag::space,boost::spirit::char_encoding::ascii> >,0>,boost::spirit::unused_type,boost::spirit::unused_type> const >,boost::fusion::cons<boost::spirit::qi::expect<boost::fusion::cons<boost::spirit::qi::literal_char<boost::spirit::char_encoding::standard,1,0>,boost::fusion::cons<boost::spirit::qi::reference<boost::spirit::qi::rule<char const *,dynamo::ast::expression __cdecl(void),boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::char_code<boost::spirit::tag::space,boost::spirit::char_encoding::ascii> >,0>,boost::spirit::unused_type,boost::spirit::unused_type> const >,boost::fusion::cons<boost::spirit::qi::literal_char<boost::spirit::char_encoding::standard,1,0>,boost::fusion::nil_> > > >,boost::fusion::nil_> > > > const >",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "fusion",
            "extension",
            "begin_impl<boost::fusion::cons_tag>",
            "apply<boost::fusion::cons<boost::spirit::qi::any_real_parser<float,boost::spirit::qi::real_policies<float> >,boost::fusion::cons<boost::spirit::qi::reference<boost::spirit::qi::rule<char const *,dynamo::ast::intrinsic_op __cdecl(void),boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::char_code<boost::spirit::tag::space,boost::spirit::char_encoding::ascii> >,0>,boost::spirit::unused_type,boost::spirit::unused_type> const >,boost::fusion::cons<boost::spirit::qi::reference<boost::spirit::qi::rule<char const *,dynamo::ast::identifier __cdecl(void),boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::char_code<boost::spirit::tag::space,boost::spirit::char_encoding::ascii> >,0>,boost::spirit::unused_type,boost::spirit::unused_type> const >,boost::fusion::cons<boost::spirit::qi::expect<boost::fusion::cons<boost::spirit::qi::literal_char<boost::spirit::char_encoding::standard,1,0>,boost::fusion::cons<boost::spirit::qi::reference<boost::spirit::qi::rule<char const *,dynamo::ast::expression __cdecl(void),boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::char_code<boost::spirit::tag::space,boost::spirit::char_encoding::ascii> >,0>,boost::spirit::unused_type,boost::spirit::unused_type> const >,boost::fusion::cons<boost::spirit::qi::literal_char<boost::spirit::char_encoding::standard,1,0>,boost::fusion::nil_> > > >,boost::fusion::nil_> > > > const >"
        ]
    );
}

#[test]
fn test_type_173() {
    let result = parse_type(
        "struct __cppobj boost::fusion::extension::deref_impl<boost::fusion::struct_iterator_tag>::apply<boost::fusion::basic_iterator<boost::fusion::struct_iterator_tag,boost::fusion::random_access_traversal_tag,dynamo::ast::assignment,0> >",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "fusion",
            "extension",
            "deref_impl<boost::fusion::struct_iterator_tag>",
            "apply<boost::fusion::basic_iterator<boost::fusion::struct_iterator_tag,boost::fusion::random_access_traversal_tag,dynamo::ast::assignment,0> >"
        ]
    );
}

#[test]
fn test_type_174() {
    let result = parse_type(
        "struct __cppobj boost::fusion::extension::end_impl<boost::fusion::cons_tag>::apply<boost::fusion::cons<boost::spirit::qi::reference<boost::spirit::qi::symbols<char,enum dynamo::ast::op_token,boost::spirit::qi::tst<char,enum dynamo::ast::op_token>,boost::spirit::qi::tst_pass_through> >,boost::fusion::cons<boost::spirit::qi::not_predicate<boost::spirit::qi::alternative<boost::fusion::cons<boost::spirit::qi::char_class<boost::spirit::tag::char_code<boost::spirit::tag::alnum,boost::spirit::char_encoding::standard> >,boost::fusion::cons<boost::spirit::qi::literal_char<boost::spirit::char_encoding::standard,1,0>,boost::fusion::nil_> > > >,boost::fusion::nil_> > >",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "fusion",
            "extension",
            "end_impl<boost::fusion::cons_tag>",
            "apply<boost::fusion::cons<boost::spirit::qi::reference<boost::spirit::qi::symbols<char,enum dynamo::ast::op_token,boost::spirit::qi::tst<char,enum dynamo::ast::op_token>,boost::spirit::qi::tst_pass_through> >,boost::fusion::cons<boost::spirit::qi::not_predicate<boost::spirit::qi::alternative<boost::fusion::cons<boost::spirit::qi::char_class<boost::spirit::tag::char_code<boost::spirit::tag::alnum,boost::spirit::char_encoding::standard> >,boost::fusion::cons<boost::spirit::qi::literal_char<boost::spirit::char_encoding::standard,1,0>,boost::fusion::nil_> > > >,boost::fusion::nil_> > >"
        ]
    );
}

#[test]
fn test_type_175() {
    let result = parse_type(
        "struct __cppobj boost::fusion::iterator_base<boost::fusion::cons_iterator<boost::fusion::cons<boost::spirit::qi::char_class<boost::spirit::tag::char_code<boost::spirit::tag::alnum,boost::spirit::char_encoding::standard> >,boost::fusion::cons<boost::spirit::qi::literal_char<boost::spirit::char_encoding::standard,1,0>,boost::fusion::nil_> > const > > : boost::fusion::iterator_root",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "fusion",
            "iterator_base<boost::fusion::cons_iterator<boost::fusion::cons<boost::spirit::qi::char_class<boost::spirit::tag::char_code<boost::spirit::tag::alnum,boost::spirit::char_encoding::standard> >,boost::fusion::cons<boost::spirit::qi::literal_char<boost::spirit::char_encoding::standard,1,0>,boost::fusion::nil_> > const > >"
        ]
    );
}

#[test]
fn test_type_176() {
    let result = parse_type(
        "struct __cppobj boost::fusion::sequence_base<boost::fusion::cons<dynamo::ast::intrinsic_op &,boost::fusion::nil_> >",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "fusion",
            "sequence_base<boost::fusion::cons<dynamo::ast::intrinsic_op &,boost::fusion::nil_> >"
        ]
    );
}

#[test]
fn test_type_177() {
    let result = parse_type(
        "struct __cppobj boost::fusion::vector1<enum dynamo::ast::op_token &> : boost::fusion::vector_data1<enum dynamo::ast::op_token &>, boost::fusion::sequence_base<boost::fusion::vector1<enum dynamo::ast::op_token &> >",
    );
    assert_eq!(
        result,
        vec!["boost", "fusion", "vector1<enum dynamo::ast::op_token &>"]
    );
}

#[test]
fn test_type_178() {
    let result = parse_type(
        "struct __cppobj boost::intrusive::pointer_traits<boost::container::container_detail::pair<ava::idstring<NAnimationSystem::SAnimationIdTag,0>,std::vector<NAnimationSystem::CAnimationVariation> > *>::rebind_pointer<boost::container::container_detail::pair<ava::idstring<NAnimationSystem::SAnimationIdTag,0>,std::vector<NAnimationSystem::CAnimationVariation> > const >",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "intrusive",
            "pointer_traits<boost::container::container_detail::pair<ava::idstring<NAnimationSystem::SAnimationIdTag,0>,std::vector<NAnimationSystem::CAnimationVariation> > *>",
            "rebind_pointer<boost::container::container_detail::pair<ava::idstring<NAnimationSystem::SAnimationIdTag,0>,std::vector<NAnimationSystem::CAnimationVariation> > const >"
        ]
    );
}

#[test]
fn test_type_179() {
    let result = parse_type(
        "struct __cppobj boost::intrusive::pointer_traits<boost::container::container_detail::pair<CCallContext const *,boost::shared_ptr<CSessionInfo> > *>::rebind_pointer<void const >",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "intrusive",
            "pointer_traits<boost::container::container_detail::pair<CCallContext const *,boost::shared_ptr<CSessionInfo> > *>",
            "rebind_pointer<void const >"
        ]
    );
}

#[test]
fn test_type_180() {
    let result = parse_type(
        "struct __cppobj boost::intrusive::pointer_traits<boost::container::container_detail::pair<CHashString,std::vector<CHashString> > *>::rebind_pointer<void const >",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "intrusive",
            "pointer_traits<boost::container::container_detail::pair<CHashString,std::vector<CHashString> > *>",
            "rebind_pointer<void const >"
        ]
    );
}

#[test]
fn test_type_181() {
    let result = parse_type(
        "struct __cppobj boost::intrusive::pointer_traits<boost::container::container_detail::pair<int,int> *>::rebind_pointer<boost::container::container_detail::pair<int,int> const >",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "intrusive",
            "pointer_traits<boost::container::container_detail::pair<int,int> *>",
            "rebind_pointer<boost::container::container_detail::pair<int,int> const >"
        ]
    );
}

#[test]
fn test_type_182() {
    let result = parse_type(
        "struct __cppobj boost::intrusive::pointer_traits<boost::container::container_detail::pair<PlayerId_Steam,CStatisticCollection *> *>::rebind_pointer<void const >",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "intrusive",
            "pointer_traits<boost::container::container_detail::pair<PlayerId_Steam,CStatisticCollection *> *>",
            "rebind_pointer<void const >"
        ]
    );
}

#[test]
fn test_type_183() {
    let result = parse_type(
        "struct __cppobj boost::intrusive::pointer_traits<std::pair<CCallContext const *,CStoreProvider_Steam::STransaction> *>::rebind_pointer<void>",
    );
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
fn test_type_184() {
    let result = parse_type(
        "struct __cppobj boost::intrusive::pointer_traits<std::pair<CHashString,ava::idstring<NAnimationSystem::SAnimationIdTag,0> > *>::rebind_pointer<void>",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "intrusive",
            "pointer_traits<std::pair<CHashString,ava::idstring<NAnimationSystem::SAnimationIdTag,0> > *>",
            "rebind_pointer<void>"
        ]
    );
}

#[test]
fn test_type_185() {
    let result = parse_type(
        "struct __cppobj boost::intrusive::pointer_traits<std::pair<CHashString,std::tr1::unordered_map<CStatisticContext,float,std::hash<CStatisticContext>,std::equal_to<CStatisticContext>,std::allocator<std::pair<CStatisticContext const ,float> > > > *>::rebind_pointer<void const >",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "intrusive",
            "pointer_traits<std::pair<CHashString,std::tr1::unordered_map<CStatisticContext,float,std::hash<CStatisticContext>,std::equal_to<CStatisticContext>,std::allocator<std::pair<CStatisticContext const ,float> > > > *>",
            "rebind_pointer<void const >"
        ]
    );
}

#[test]
fn test_type_186() {
    let result = parse_type(
        "struct __cppobj boost::intrusive::pointer_traits<std::pair<std::pair<ava::idstring<NAnimationSystem::SAnimationIdTag,0>,ava::idstring<NAnimationSystem::SAnimationIdTag,0> >,float> *>::rebind_pointer<void const >",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "intrusive",
            "pointer_traits<std::pair<std::pair<ava::idstring<NAnimationSystem::SAnimationIdTag,0>,ava::idstring<NAnimationSystem::SAnimationIdTag,0> >,float> *>",
            "rebind_pointer<void const >"
        ]
    );
}

#[test]
fn test_type_187() {
    let result = parse_type(
        "struct __cppobj boost::intrusive::pointer_traits<std::pair<unsigned int,TPropertyContainer<unsigned int,std::allocator<int>,CFastHashFtor> > *>",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "intrusive",
            "pointer_traits<std::pair<unsigned int,TPropertyContainer<unsigned int,std::allocator<int>,CFastHashFtor> > *>"
        ]
    );
}

#[test]
fn test_type_188() {
    let result = parse_type(
        "struct __cppobj boost::is_same<boost::fusion::cons_iterator_identity<boost::fusion::nil_ const >,boost::fusion::cons_iterator_identity<boost::fusion::nil_ const > > : boost::integral_constant<bool,1>",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "is_same<boost::fusion::cons_iterator_identity<boost::fusion::nil_ const >,boost::fusion::cons_iterator_identity<boost::fusion::nil_ const > >"
        ]
    );
}

#[test]
fn test_type_189() {
    let result = parse_type(
        "struct __cppobj boost::iterator_range_detail::iterator_range_impl<std::_String_iterator<char,std::char_traits<char>,std::allocator<char> > >",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "iterator_range_detail",
            "iterator_range_impl<std::_String_iterator<char,std::char_traits<char>,std::allocator<char> > >"
        ]
    );
}

#[test]
fn test_type_190() {
    let result = parse_type(
        "struct __cppobj boost::mpl::aux::and_impl<0,boost::spirit::traits::not_is_optional<boost::variant<dynamo::ast::assignment,dynamo::ast::expression>,boost::spirit::qi::domain,void>,boost::mpl::bool_<1>,boost::mpl::bool_<1>,boost::mpl::bool_<1> > : boost::mpl::bool_<0>",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "mpl",
            "aux",
            "and_impl<0,boost::spirit::traits::not_is_optional<boost::variant<dynamo::ast::assignment,dynamo::ast::expression>,boost::spirit::qi::domain,void>,boost::mpl::bool_<1>,boost::mpl::bool_<1>,boost::mpl::bool_<1> >"
        ]
    );
}

#[test]
fn test_type_191() {
    let result = parse_type(
        "struct __cppobj boost::mpl::aux::and_impl<0,boost::spirit::traits::pass_through_container<std::vector<dynamo::ast::expression>,dynamo::ast::expression,dynamo::ast::expression,boost::mpl::bool_<0>,boost::spirit::qi::domain,void>,boost::mpl::bool_<1>,boost::mpl::bool_<1>,boost::mpl::bool_<1> > : boost::mpl::bool_<0>",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "mpl",
            "aux",
            "and_impl<0,boost::spirit::traits::pass_through_container<std::vector<dynamo::ast::expression>,dynamo::ast::expression,dynamo::ast::expression,boost::mpl::bool_<0>,boost::spirit::qi::domain,void>,boost::mpl::bool_<1>,boost::mpl::bool_<1>,boost::mpl::bool_<1> >"
        ]
    );
}

#[test]
fn test_type_192() {
    let result = parse_type(
        "struct __cppobj boost::mpl::list3<unsigned __int64,float,std::string > : boost::mpl::l_item<boost::mpl::long_<3>,unsigned __int64,boost::mpl::list2<float,std::string > >",
    );
    assert_eq!(
        result,
        vec!["boost", "mpl", "list3<unsigned __int64,float,std::string >"]
    );
}

#[test]
fn test_type_193() {
    let result = parse_type(
        "struct __cppobj boost::optional_detail::types_when_isnt_ref<dynamo::vm::executable>",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "optional_detail",
            "types_when_isnt_ref<dynamo::vm::executable>"
        ]
    );
}

#[test]
fn test_type_194() {
    let result = parse_type(
        "struct __cppobj boost::phoenix::detail::function_eval::result<boost::phoenix::detail::function_eval __cdecl(boost::proto::exprns_::basic_expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<dynamo::annotation<char const *> >,0> const &,boost::phoenix::actor<boost::spirit::attribute<0> > const &,boost::phoenix::actor<boost::spirit::argument<0> > const &,boost::phoenix::vector2<boost::phoenix::vector3<boost::phoenix::actor<boost::proto::exprns_::basic_expr<boost::phoenix::detail::tag::function_eval,boost::proto::argsns_::list3<boost::proto::exprns_::basic_expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<dynamo::annotation<char const *> >,0>,boost::phoenix::actor<boost::spirit::attribute<0> >,boost::phoenix::actor<boost::spirit::argument<0> > >,3> > const *,boost::fusion::vector<char const * &,char const * const &,char const * const &,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_> const &,boost::spirit::context<boost::fusion::cons<dynamo::ast::assignment &,boost::fusion::nil_>,boost::fusion::vector0<void> > &> &,boost::phoenix::default_actions const &>)>",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "phoenix",
            "detail",
            "function_eval",
            "result<boost::phoenix::detail::function_eval __cdecl(boost::proto::exprns_::basic_expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<dynamo::annotation<char const *> >,0> const &,boost::phoenix::actor<boost::spirit::attribute<0> > const &,boost::phoenix::actor<boost::spirit::argument<0> > const &,boost::phoenix::vector2<boost::phoenix::vector3<boost::phoenix::actor<boost::proto::exprns_::basic_expr<boost::phoenix::detail::tag::function_eval,boost::proto::argsns_::list3<boost::proto::exprns_::basic_expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<dynamo::annotation<char const *> >,0>,boost::phoenix::actor<boost::spirit::attribute<0> >,boost::phoenix::actor<boost::spirit::argument<0> > >,3> > const *,boost::fusion::vector<char const * &,char const * const &,char const * const &,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_> const &,boost::spirit::context<boost::fusion::cons<dynamo::ast::assignment &,boost::fusion::nil_>,boost::fusion::vector0<void> > &> &,boost::phoenix::default_actions const &>)>"
        ]
    );
}

#[test]
fn test_type_195() {
    let result = parse_type(
        "struct __cppobj boost::phoenix::evaluator::impl<boost::proto::exprns_::basic_expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<dynamo::error_handler<char const *> >,0> const &,boost::phoenix::vector2<boost::phoenix::vector4<boost::phoenix::actor<boost::proto::exprns_::basic_expr<boost::phoenix::detail::tag::function_eval,boost::proto::argsns_::list4<boost::proto::exprns_::basic_expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<dynamo::error_handler<char const *> >,0>,boost::proto::exprns_::basic_expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<char const *>,0>,boost::phoenix::actor<boost::spirit::argument<3> >,boost::phoenix::actor<boost::spirit::argument<2> > >,4> > const *,boost::fusion::vector<char const * &,char const * const &,char const * const &,boost::spirit::info const &,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_> &,boost::spirit::context<boost::fusion::cons<boost::variant<dynamo::ast::nil,dynamo::ast::assignment,dynamo::ast::expression> &,boost::fusion::nil_>,boost::fusion::vector0<void> > &,enum boost::spirit::qi::error_handler_result &> &,boost::phoenix::default_actions const &> const &,boost::proto::envns_::empty_env> : boost::proto::transform_impl<boost::proto::exprns_::basic_expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<dynamo::error_handler<char const *> >,0> const &,boost::phoenix::vector2<boost::phoenix::vector4<boost::phoenix::actor<boost::proto::exprns_::basic_expr<boost::phoenix::detail::tag::function_eval,boost::proto::argsns_::list4<boost::proto::exprns_::basic_expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<dynamo::error_handler<char const *> >,0>,boost::proto::exprns_::basic_expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<char const *>,0>,boost::phoenix::actor<boost::spirit::argument<3> >,boost::phoenix::actor<boost::spirit::argument<2> > >,4> > const *,boost::fusion::vector<char const * &,char const * const &,char const * const &,boost::spirit::info const &,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_> &,boost::spirit::context<boost::fusion::cons<boost::variant<dynamo::ast::nil,dynamo::ast::assignment,dynamo::ast::expression> &,boost::fusion::nil_>,boost::fusion::vector0<void> > &,enum boost::spirit::qi::error_handler_result &> &,boost::phoenix::default_actions const &> const &,boost::proto::envns_::empty_env>",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "phoenix",
            "evaluator",
            "impl<boost::proto::exprns_::basic_expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<dynamo::error_handler<char const *> >,0> const &,boost::phoenix::vector2<boost::phoenix::vector4<boost::phoenix::actor<boost::proto::exprns_::basic_expr<boost::phoenix::detail::tag::function_eval,boost::proto::argsns_::list4<boost::proto::exprns_::basic_expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<dynamo::error_handler<char const *> >,0>,boost::proto::exprns_::basic_expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<char const *>,0>,boost::phoenix::actor<boost::spirit::argument<3> >,boost::phoenix::actor<boost::spirit::argument<2> > >,4> > const *,boost::fusion::vector<char const * &,char const * const &,char const * const &,boost::spirit::info const &,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_> &,boost::spirit::context<boost::fusion::cons<boost::variant<dynamo::ast::nil,dynamo::ast::assignment,dynamo::ast::expression> &,boost::fusion::nil_>,boost::fusion::vector0<void> > &,enum boost::spirit::qi::error_handler_result &> &,boost::phoenix::default_actions const &> const &,boost::proto::envns_::empty_env>"
        ]
    );
}

#[test]
fn test_type_196() {
    let result = parse_type(
        "struct __cppobj boost::phoenix::evaluator::result<boost::phoenix::evaluator __cdecl(boost::phoenix::actor<boost::spirit::argument<3> > const &,boost::phoenix::vector2<boost::phoenix::vector4<boost::phoenix::actor<boost::proto::exprns_::basic_expr<boost::phoenix::detail::tag::function_eval,boost::proto::argsns_::list4<boost::proto::exprns_::basic_expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<dynamo::error_handler<char const *> >,0>,boost::proto::exprns_::basic_expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<char const *>,0>,boost::phoenix::actor<boost::spirit::argument<3> >,boost::phoenix::actor<boost::spirit::argument<2> > >,4> > const *,boost::fusion::vector<char const * &,char const * const &,char const * const &,boost::spirit::info const &,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_> const &,boost::spirit::context<boost::fusion::cons<dynamo::ast::expression &,boost::fusion::nil_>,boost::fusion::vector0<void> > const &,enum boost::spirit::qi::error_handler_result const &> &,boost::phoenix::default_actions const &>)>",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "phoenix",
            "evaluator",
            "result<boost::phoenix::evaluator __cdecl(boost::phoenix::actor<boost::spirit::argument<3> > const &,boost::phoenix::vector2<boost::phoenix::vector4<boost::phoenix::actor<boost::proto::exprns_::basic_expr<boost::phoenix::detail::tag::function_eval,boost::proto::argsns_::list4<boost::proto::exprns_::basic_expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<dynamo::error_handler<char const *> >,0>,boost::proto::exprns_::basic_expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<char const *>,0>,boost::phoenix::actor<boost::spirit::argument<3> >,boost::phoenix::actor<boost::spirit::argument<2> > >,4> > const *,boost::fusion::vector<char const * &,char const * const &,char const * const &,boost::spirit::info const &,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_> const &,boost::spirit::context<boost::fusion::cons<dynamo::ast::expression &,boost::fusion::nil_>,boost::fusion::vector0<void> > const &,enum boost::spirit::qi::error_handler_result const &> &,boost::phoenix::default_actions const &>)>"
        ]
    );
}

#[test]
fn test_type_197() {
    let result = parse_type(
        "struct __cppobj boost::phoenix::is_nullary::when<boost::phoenix::rule::terminal,void>::impl<boost::proto::exprns_::basic_expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<char const *>,0> const &,boost::phoenix::vector2<boost::mpl::bool_<1>,boost::phoenix::is_nullary &>,boost::phoenix::is_nullary &>",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "phoenix",
            "is_nullary",
            "when<boost::phoenix::rule::terminal,void>",
            "impl<boost::proto::exprns_::basic_expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<char const *>,0> const &,boost::phoenix::vector2<boost::mpl::bool_<1>,boost::phoenix::is_nullary &>,boost::phoenix::is_nullary &>"
        ]
    );
}

#[test]
fn test_type_198() {
    let result = parse_type(
        "struct __cppobj boost::phoenix::result_of::env<boost::phoenix::vector2<boost::phoenix::vector4<boost::phoenix::actor<boost::proto::exprns_::basic_expr<boost::phoenix::detail::tag::function_eval,boost::proto::argsns_::list4<boost::proto::exprns_::basic_expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<dynamo::error_handler<char const *> >,0>,boost::proto::exprns_::basic_expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<char const *>,0>,boost::phoenix::actor<boost::spirit::argument<3> >,boost::phoenix::actor<boost::spirit::argument<2> > >,4> > const *,boost::fusion::vector<char const * &,char const * const &,char const * const &,boost::spirit::info const &,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_> &,boost::spirit::context<boost::fusion::cons<boost::variant<dynamo::ast::nil,dynamo::ast::assignment,dynamo::ast::expression> &,boost::fusion::nil_>,boost::fusion::vector0<void> > &,enum boost::spirit::qi::error_handler_result &> &,boost::phoenix::default_actions const &> const >",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "phoenix",
            "result_of",
            "env<boost::phoenix::vector2<boost::phoenix::vector4<boost::phoenix::actor<boost::proto::exprns_::basic_expr<boost::phoenix::detail::tag::function_eval,boost::proto::argsns_::list4<boost::proto::exprns_::basic_expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<dynamo::error_handler<char const *> >,0>,boost::proto::exprns_::basic_expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<char const *>,0>,boost::phoenix::actor<boost::spirit::argument<3> >,boost::phoenix::actor<boost::spirit::argument<2> > >,4> > const *,boost::fusion::vector<char const * &,char const * const &,char const * const &,boost::spirit::info const &,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_> &,boost::spirit::context<boost::fusion::cons<boost::variant<dynamo::ast::nil,dynamo::ast::assignment,dynamo::ast::expression> &,boost::fusion::nil_>,boost::fusion::vector0<void> > &,enum boost::spirit::qi::error_handler_result &> &,boost::phoenix::default_actions const &> const >"
        ]
    );
}

#[test]
fn test_type_199() {
    let result = parse_type(
        "struct __cppobj boost::proto::_value::impl<boost::phoenix::actor<boost::spirit::argument<2> > const &,boost::phoenix::vector4<boost::phoenix::actor<boost::proto::exprns_::basic_expr<boost::phoenix::detail::tag::function_eval,boost::proto::argsns_::list4<boost::proto::exprns_::basic_expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<dynamo::error_handler<char const *> >,0>,boost::proto::exprns_::basic_expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<char const *>,0>,boost::phoenix::actor<boost::spirit::argument<3> >,boost::phoenix::actor<boost::spirit::argument<2> > >,4> > const *,boost::fusion::vector<char const * &,char const * const &,char const * const &,boost::spirit::info const &,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_> const &,boost::spirit::context<boost::fusion::cons<boost::variant<dynamo::ast::nil,dynamo::ast::assignment,dynamo::ast::expression> &,boost::fusion::nil_>,boost::fusion::vector0<void> > const &,enum boost::spirit::qi::error_handler_result const &> &,boost::phoenix::default_actions const &> : boost::proto::transform_impl<boost::phoenix::actor<boost::spirit::argument<2> > const &,boost::phoenix::vector4<boost::phoenix::actor<boost::proto::exprns_::basic_expr<boost::phoenix::detail::tag::function_eval,boost::proto::argsns_::list4<boost::proto::exprns_::basic_expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<dynamo::error_handler<char const *> >,0>,boost::proto::exprns_::basic_expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<char const *>,0>,boost::phoenix::actor<boost::spirit::argument<3> >,boost::phoenix::actor<boost::spirit::argument<2> > >,4> > const *,boost::fusion::vector<char const * &,char const * const &,char const * const &,boost::spirit::info const &,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_> const &,boost::spirit::context<boost::fusion::cons<boost::variant<dynamo::ast::nil,dynamo::ast::assignment,dynamo::ast::expression> &,boost::fusion::nil_>,boost::fusion::vector0<void> > const &,enum boost::spirit::qi::error_handler_result const &> &,boost::phoenix::default_actions const &>",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "proto",
            "_value",
            "impl<boost::phoenix::actor<boost::spirit::argument<2> > const &,boost::phoenix::vector4<boost::phoenix::actor<boost::proto::exprns_::basic_expr<boost::phoenix::detail::tag::function_eval,boost::proto::argsns_::list4<boost::proto::exprns_::basic_expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<dynamo::error_handler<char const *> >,0>,boost::proto::exprns_::basic_expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<char const *>,0>,boost::phoenix::actor<boost::spirit::argument<3> >,boost::phoenix::actor<boost::spirit::argument<2> > >,4> > const *,boost::fusion::vector<char const * &,char const * const &,char const * const &,boost::spirit::info const &,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_> const &,boost::spirit::context<boost::fusion::cons<boost::variant<dynamo::ast::nil,dynamo::ast::assignment,dynamo::ast::expression> &,boost::fusion::nil_>,boost::fusion::vector0<void> > const &,enum boost::spirit::qi::error_handler_result const &> &,boost::phoenix::default_actions const &>"
        ]
    );
}

#[test]
fn test_type_200() {
    let result = parse_type(
        "struct __cppobj boost::proto::_value::impl<boost::proto::exprns_::basic_expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::phoenix::detail::local<boost::phoenix::local_names::_a_key> >,0>,int,int> : boost::proto::transform_impl<boost::proto::exprns_::basic_expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::phoenix::detail::local<boost::phoenix::local_names::_a_key> >,0>,int,int>",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "proto",
            "_value",
            "impl<boost::proto::exprns_::basic_expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::phoenix::detail::local<boost::phoenix::local_names::_a_key> >,0>,int,int>"
        ]
    );
}

#[test]
fn test_type_201() {
    let result = parse_type(
        "struct __cppobj boost::proto::call<boost::phoenix::custom_terminal<boost::spirit::attribute<0>,void> __cdecl(boost::proto::_value,boost::phoenix::_context)> : boost::proto::transform<boost::proto::call<boost::phoenix::custom_terminal<boost::spirit::attribute<0>,void> __cdecl(boost::proto::_value,boost::phoenix::_context)>,void>",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "proto",
            "call<boost::phoenix::custom_terminal<boost::spirit::attribute<0>,void> __cdecl(boost::proto::_value,boost::phoenix::_context)>"
        ]
    );
}

#[test]
fn test_type_202() {
    let result = parse_type(
        "struct __cppobj boost::proto::detail::apply_transform<boost::phoenix::evaluator __cdecl(boost::phoenix::actor<boost::spirit::attribute<0> > const &,boost::phoenix::vector2<boost::phoenix::vector3<boost::phoenix::actor<boost::proto::exprns_::basic_expr<boost::phoenix::detail::tag::function_eval,boost::proto::argsns_::list3<boost::proto::exprns_::basic_expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<dynamo::annotation<char const *> >,0>,boost::phoenix::actor<boost::spirit::attribute<0> >,boost::phoenix::actor<boost::spirit::argument<0> > >,3> > const *,boost::fusion::vector<char const * &,char const * const &,char const * const &,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_> &,boost::spirit::context<boost::fusion::cons<dynamo::ast::assignment &,boost::fusion::nil_>,boost::fusion::vector0<void> > &> &,boost::phoenix::default_actions const &> const &)> : boost::phoenix::evaluator::impl<boost::phoenix::actor<boost::spirit::attribute<0> > const &,boost::phoenix::vector2<boost::phoenix::vector3<boost::phoenix::actor<boost::proto::exprns_::basic_expr<boost::phoenix::detail::tag::function_eval,boost::proto::argsns_::list3<boost::proto::exprns_::basic_expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<dynamo::annotation<char const *> >,0>,boost::phoenix::actor<boost::spirit::attribute<0> >,boost::phoenix::actor<boost::spirit::argument<0> > >,3> > const *,boost::fusion::vector<char const * &,char const * const &,char const * const &,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_> &,boost::spirit::context<boost::fusion::cons<dynamo::ast::assignment &,boost::fusion::nil_>,boost::fusion::vector0<void> > &> &,boost::phoenix::default_actions const &> const &,boost::proto::envns_::empty_env>",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "proto",
            "detail",
            "apply_transform<boost::phoenix::evaluator __cdecl(boost::phoenix::actor<boost::spirit::attribute<0> > const &,boost::phoenix::vector2<boost::phoenix::vector3<boost::phoenix::actor<boost::proto::exprns_::basic_expr<boost::phoenix::detail::tag::function_eval,boost::proto::argsns_::list3<boost::proto::exprns_::basic_expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<dynamo::annotation<char const *> >,0>,boost::phoenix::actor<boost::spirit::attribute<0> >,boost::phoenix::actor<boost::spirit::argument<0> > >,3> > const *,boost::fusion::vector<char const * &,char const * const &,char const * const &,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_> &,boost::spirit::context<boost::fusion::cons<dynamo::ast::assignment &,boost::fusion::nil_>,boost::fusion::vector0<void> > &> &,boost::phoenix::default_actions const &> const &)>"
        ]
    );
}

#[test]
fn test_type_203() {
    let result = parse_type(
        "struct __cppobj boost::proto::detail::apply_transform<boost::proto::switch_<boost::spirit::meta_compiler<boost::spirit::qi::domain>::cases,boost::proto::tag_of<boost::proto::_> __cdecl(void)> __cdecl(boost::spirit::qi::symbols<char,enum dynamo::ast::op_token,boost::spirit::qi::tst<char,enum dynamo::ast::op_token>,boost::spirit::qi::tst_pass_through> &,boost::fusion::cons<boost::spirit::qi::not_predicate<boost::spirit::qi::alternative<boost::fusion::cons<boost::spirit::qi::char_class<boost::spirit::tag::char_code<boost::spirit::tag::alnum,boost::spirit::char_encoding::standard> >,boost::fusion::cons<boost::spirit::qi::literal_char<boost::spirit::char_encoding::standard,1,0>,boost::fusion::nil_> > > >,boost::fusion::nil_> const &,boost::spirit::unused_type &)> : boost::proto::switch_<boost::spirit::meta_compiler<boost::spirit::qi::domain>::cases,boost::proto::tag_of<boost::proto::_> __cdecl(void)>::impl<boost::spirit::qi::symbols<char,enum dynamo::ast::op_token,boost::spirit::qi::tst<char,enum dynamo::ast::op_token>,boost::spirit::qi::tst_pass_through> &,boost::fusion::cons<boost::spirit::qi::not_predicate<boost::spirit::qi::alternative<boost::fusion::cons<boost::spirit::qi::char_class<boost::spirit::tag::char_code<boost::spirit::tag::alnum,boost::spirit::char_encoding::standard> >,boost::fusion::cons<boost::spirit::qi::literal_char<boost::spirit::char_encoding::standard,1,0>,boost::fusion::nil_> > > >,boost::fusion::nil_> const &,boost::spirit::unused_type &>",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "proto",
            "detail",
            "apply_transform<boost::proto::switch_<boost::spirit::meta_compiler<boost::spirit::qi::domain>::cases,boost::proto::tag_of<boost::proto::_> __cdecl(void)> __cdecl(boost::spirit::qi::symbols<char,enum dynamo::ast::op_token,boost::spirit::qi::tst<char,enum dynamo::ast::op_token>,boost::spirit::qi::tst_pass_through> &,boost::fusion::cons<boost::spirit::qi::not_predicate<boost::spirit::qi::alternative<boost::fusion::cons<boost::spirit::qi::char_class<boost::spirit::tag::char_code<boost::spirit::tag::alnum,boost::spirit::char_encoding::standard> >,boost::fusion::cons<boost::spirit::qi::literal_char<boost::spirit::char_encoding::standard,1,0>,boost::fusion::nil_> > > >,boost::fusion::nil_> const &,boost::spirit::unused_type &)>"
        ]
    );
}

#[test]
fn test_type_204() {
    let result = parse_type(
        "struct __cppobj boost::proto::detail::protoify<boost::phoenix::actor<boost::spirit::argument<0> > const ,boost::proto::domainns_::basic_default_domain> : boost::proto::domainns_::domain<boost::proto::basic_default_generator,boost::proto::_,boost::proto::detail::not_a_domain>::as_expr<boost::phoenix::actor<boost::spirit::argument<0> > const ,void,boost::proto::callable>",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "proto",
            "detail",
            "protoify<boost::phoenix::actor<boost::spirit::argument<0> > const ,boost::proto::domainns_::basic_default_domain>"
        ]
    );
}

#[test]
fn test_type_205() {
    let result = parse_type(
        "struct __cppobj boost::proto::domainns_::domain<boost::proto::basic_default_generator,boost::proto::_,boost::proto::detail::not_a_domain> : boost::proto::basic_default_generator",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "proto",
            "domainns_",
            "domain<boost::proto::basic_default_generator,boost::proto::_,boost::proto::detail::not_a_domain>"
        ]
    );
}

#[test]
fn test_type_206() {
    let result = parse_type(
        "struct __cppobj boost::proto::domainns_::domain<boost::proto::default_generator,boost::proto::_,boost::proto::detail::not_a_domain>::as_child<boost::spirit::qi::rule<char const *,dynamo::ast::expression __cdecl(void),boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::char_code<boost::spirit::tag::space,boost::spirit::char_encoding::ascii> >,0>,boost::spirit::unused_type,boost::spirit::unused_type>,void,boost::proto::callable>",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "proto",
            "domainns_",
            "domain<boost::proto::default_generator,boost::proto::_,boost::proto::detail::not_a_domain>",
            "as_child<boost::spirit::qi::rule<char const *,dynamo::ast::expression __cdecl(void),boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::char_code<boost::spirit::tag::space,boost::spirit::char_encoding::ascii> >,0>,boost::spirit::unused_type,boost::spirit::unused_type>,void,boost::proto::callable>"
        ]
    );
}

#[test]
fn test_type_207() {
    let result = parse_type(
        "struct __cppobj boost::proto::exprns_::extends<boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::long_long>,0>,boost::spirit::terminal<boost::spirit::tag::long_long>,boost::proto::domainns_::default_domain,0>",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "proto",
            "exprns_",
            "extends<boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::long_long>,0>,boost::spirit::terminal<boost::spirit::tag::long_long>,boost::proto::domainns_::default_domain,0>"
        ]
    );
}

#[test]
fn test_type_208() {
    let result = parse_type(
        "struct __cppobj boost::proto::if_<boost::proto::detail::has_tag<boost::proto::tagns_::tag::bitwise_or>,boost::proto::reverse_fold<boost::proto::_,boost::proto::_state,boost::proto::detail::reverse_fold_tree_<boost::proto::tagns_::tag::bitwise_or,boost::spirit::detail::make_binary_helper<boost::spirit::meta_compiler<boost::spirit::qi::domain>::meta_grammar> > >,boost::spirit::detail::make_binary_helper<boost::spirit::meta_compiler<boost::spirit::qi::domain>::meta_grammar> >::impl<boost::spirit::qi::rule<char const *,dynamo::ast::intrinsic_op __cdecl(void),boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::char_code<boost::spirit::tag::space,boost::spirit::char_encoding::ascii> >,0>,boost::spirit::unused_type,boost::spirit::unused_type>,boost::fusion::cons<boost::spirit::qi::reference<boost::spirit::qi::rule<char const *,dynamo::ast::identifier __cdecl(void),boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::char_code<boost::spirit::tag::space,boost::spirit::char_encoding::ascii> >,0>,boost::spirit::unused_type,boost::spirit::unused_type> const >,boost::fusion::cons<boost::spirit::qi::expect<boost::fusion::cons<boost::spirit::qi::literal_char<boost::spirit::char_encoding::standard,1,0>,boost::fusion::cons<boost::spirit::qi::reference<boost::spirit::qi::rule<char const *,dynamo::ast::expression __cdecl(void),boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::char_code<boost::spirit::tag::space,boost::spirit::char_encoding::ascii> >,0>,boost::spirit::unused_type,boost::spirit::unused_type> const >,boost::fusion::cons<boost::spirit::qi::literal_char<boost::spirit::char_encoding::standard,1,0>,boost::fusion::nil_> > > >,boost::fusion::nil_> >,boost::spirit::unused_type> : boost::proto::transform_impl<boost::spirit::qi::rule<char const *,dynamo::ast::intrinsic_op __cdecl(void),boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::char_code<boost::spirit::tag::space,boost::spirit::char_encoding::ascii> >,0>,boost::spirit::unused_type,boost::spirit::unused_type>,boost::fusion::cons<boost::spirit::qi::reference<boost::spirit::qi::rule<char const *,dynamo::ast::identifier __cdecl(void),boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::char_code<boost::spirit::tag::space,boost::spirit::char_encoding::ascii> >,0>,boost::spirit::unused_type,boost::spirit::unused_type> const >,boost::fusion::cons<boost::spirit::qi::expect<boost::fusion::cons<boost::spirit::qi::literal_char<boost::spirit::char_encoding::standard,1,0>,boost::fusion::cons<boost::spirit::qi::reference<boost::spirit::qi::rule<char const *,dynamo::ast::expression __cdecl(void),boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::char_code<boost::spirit::tag::space,boost::spirit::char_encoding::ascii> >,0>,boost::spirit::unused_type,boost::spirit::unused_type> const >,boost::fusion::cons<boost::spirit::qi::literal_char<boost::spirit::char_encoding::standard,1,0>,boost::fusion::nil_> > > >,boost::fusion::nil_> >,boost::spirit::unused_type>",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "proto",
            "if_<boost::proto::detail::has_tag<boost::proto::tagns_::tag::bitwise_or>,boost::proto::reverse_fold<boost::proto::_,boost::proto::_state,boost::proto::detail::reverse_fold_tree_<boost::proto::tagns_::tag::bitwise_or,boost::spirit::detail::make_binary_helper<boost::spirit::meta_compiler<boost::spirit::qi::domain>::meta_grammar> > >,boost::spirit::detail::make_binary_helper<boost::spirit::meta_compiler<boost::spirit::qi::domain>::meta_grammar> >",
            "impl<boost::spirit::qi::rule<char const *,dynamo::ast::intrinsic_op __cdecl(void),boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::char_code<boost::spirit::tag::space,boost::spirit::char_encoding::ascii> >,0>,boost::spirit::unused_type,boost::spirit::unused_type>,boost::fusion::cons<boost::spirit::qi::reference<boost::spirit::qi::rule<char const *,dynamo::ast::identifier __cdecl(void),boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::char_code<boost::spirit::tag::space,boost::spirit::char_encoding::ascii> >,0>,boost::spirit::unused_type,boost::spirit::unused_type> const >,boost::fusion::cons<boost::spirit::qi::expect<boost::fusion::cons<boost::spirit::qi::literal_char<boost::spirit::char_encoding::standard,1,0>,boost::fusion::cons<boost::spirit::qi::reference<boost::spirit::qi::rule<char const *,dynamo::ast::expression __cdecl(void),boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::char_code<boost::spirit::tag::space,boost::spirit::char_encoding::ascii> >,0>,boost::spirit::unused_type,boost::spirit::unused_type> const >,boost::fusion::cons<boost::spirit::qi::literal_char<boost::spirit::char_encoding::standard,1,0>,boost::fusion::nil_> > > >,boost::fusion::nil_> >,boost::spirit::unused_type>"
        ]
    );
}

#[test]
fn test_type_209() {
    let result = parse_type(
        "struct __cppobj boost::proto::make<boost::spirit::argument<2> __cdecl(void)>::impl<boost::spirit::argument<2> const &,boost::phoenix::vector2<boost::phoenix::vector3<boost::phoenix::actor<boost::proto::exprns_::basic_expr<boost::phoenix::detail::tag::function_eval,boost::proto::argsns_::list3<boost::proto::exprns_::basic_expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<dynamo::annotation<char const *> >,0>,boost::phoenix::actor<boost::spirit::attribute<0> >,boost::phoenix::actor<boost::spirit::argument<2> > >,3> > const *,boost::fusion::vector<char const * &,char const * const &,char const * const &,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_> const &,boost::spirit::context<boost::fusion::cons<dynamo::ast::intrinsic_op &,boost::fusion::nil_>,boost::fusion::vector0<void> > &> &,boost::phoenix::default_actions const &>,boost::phoenix::default_actions const &> : boost::proto::transform_impl<boost::spirit::argument<2> const &,boost::phoenix::vector2<boost::phoenix::vector3<boost::phoenix::actor<boost::proto::exprns_::basic_expr<boost::phoenix::detail::tag::function_eval,boost::proto::argsns_::list3<boost::proto::exprns_::basic_expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<dynamo::annotation<char const *> >,0>,boost::phoenix::actor<boost::spirit::attribute<0> >,boost::phoenix::actor<boost::spirit::argument<2> > >,3> > const *,boost::fusion::vector<char const * &,char const * const &,char const * const &,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_> const &,boost::spirit::context<boost::fusion::cons<dynamo::ast::intrinsic_op &,boost::fusion::nil_>,boost::fusion::vector0<void> > &> &,boost::phoenix::default_actions const &>,boost::phoenix::default_actions const &>",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "proto",
            "make<boost::spirit::argument<2> __cdecl(void)>",
            "impl<boost::spirit::argument<2> const &,boost::phoenix::vector2<boost::phoenix::vector3<boost::phoenix::actor<boost::proto::exprns_::basic_expr<boost::phoenix::detail::tag::function_eval,boost::proto::argsns_::list3<boost::proto::exprns_::basic_expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<dynamo::annotation<char const *> >,0>,boost::phoenix::actor<boost::spirit::attribute<0> >,boost::phoenix::actor<boost::spirit::argument<2> > >,3> > const *,boost::fusion::vector<char const * &,char const * const &,char const * const &,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_> const &,boost::spirit::context<boost::fusion::cons<dynamo::ast::intrinsic_op &,boost::fusion::nil_>,boost::fusion::vector0<void> > &> &,boost::phoenix::default_actions const &>,boost::phoenix::default_actions const &>"
        ]
    );
}

#[test]
fn test_type_210() {
    let result = parse_type(
        "struct __cppobj boost::proto::make<boost::spirit::attribute<0> __cdecl(void)>::impl<boost::spirit::attribute<0> const &,boost::phoenix::vector2<boost::phoenix::vector3<boost::phoenix::actor<boost::proto::exprns_::basic_expr<boost::phoenix::detail::tag::function_eval,boost::proto::argsns_::list3<boost::proto::exprns_::basic_expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<dynamo::annotation<char const *> >,0>,boost::phoenix::actor<boost::spirit::attribute<0> >,boost::phoenix::actor<boost::spirit::argument<0> > >,3> > const *,boost::fusion::vector<char const * &,char const * const &,char const * const &,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_> const &,boost::spirit::context<boost::fusion::cons<dynamo::ast::assignment &,boost::fusion::nil_>,boost::fusion::vector0<void> > const &> &,boost::phoenix::default_actions const &>,boost::phoenix::default_actions const &> : boost::proto::transform_impl<boost::spirit::attribute<0> const &,boost::phoenix::vector2<boost::phoenix::vector3<boost::phoenix::actor<boost::proto::exprns_::basic_expr<boost::phoenix::detail::tag::function_eval,boost::proto::argsns_::list3<boost::proto::exprns_::basic_expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<dynamo::annotation<char const *> >,0>,boost::phoenix::actor<boost::spirit::attribute<0> >,boost::phoenix::actor<boost::spirit::argument<0> > >,3> > const *,boost::fusion::vector<char const * &,char const * const &,char const * const &,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_> const &,boost::spirit::context<boost::fusion::cons<dynamo::ast::assignment &,boost::fusion::nil_>,boost::fusion::vector0<void> > const &> &,boost::phoenix::default_actions const &>,boost::phoenix::default_actions const &>",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "proto",
            "make<boost::spirit::attribute<0> __cdecl(void)>",
            "impl<boost::spirit::attribute<0> const &,boost::phoenix::vector2<boost::phoenix::vector3<boost::phoenix::actor<boost::proto::exprns_::basic_expr<boost::phoenix::detail::tag::function_eval,boost::proto::argsns_::list3<boost::proto::exprns_::basic_expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<dynamo::annotation<char const *> >,0>,boost::phoenix::actor<boost::spirit::attribute<0> >,boost::phoenix::actor<boost::spirit::argument<0> > >,3> > const *,boost::fusion::vector<char const * &,char const * const &,char const * const &,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_> const &,boost::spirit::context<boost::fusion::cons<dynamo::ast::assignment &,boost::fusion::nil_>,boost::fusion::vector0<void> > const &> &,boost::phoenix::default_actions const &>,boost::phoenix::default_actions const &>"
        ]
    );
}

#[test]
fn test_type_211() {
    let result = parse_type(
        "struct __cppobj boost::proto::or_<boost::phoenix::enable_rule<boost::phoenix::rule::argument,void>,boost::phoenix::enable_rule<boost::phoenix::rule::custom_terminal,void>,boost::phoenix::enable_rule<boost::phoenix::rule::terminal,void>,void,void,void,void,void,void,void>::impl<boost::proto::exprns_::basic_expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::phoenix::detail::local<boost::phoenix::local_names::_j_key> >,0> const &,boost::mpl::bool_<1> &,boost::phoenix::is_nullary &> : boost::proto::when<boost::phoenix::rule::custom_terminal,boost::proto::external_transform>::impl<boost::proto::exprns_::basic_expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::phoenix::detail::local<boost::phoenix::local_names::_j_key> >,0> const &,boost::mpl::bool_<1> &,boost::phoenix::is_nullary &>",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "proto",
            "or_<boost::phoenix::enable_rule<boost::phoenix::rule::argument,void>,boost::phoenix::enable_rule<boost::phoenix::rule::custom_terminal,void>,boost::phoenix::enable_rule<boost::phoenix::rule::terminal,void>,void,void,void,void,void,void,void>",
            "impl<boost::proto::exprns_::basic_expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::phoenix::detail::local<boost::phoenix::local_names::_j_key> >,0> const &,boost::mpl::bool_<1> &,boost::phoenix::is_nullary &>"
        ]
    );
}

#[test]
fn test_type_212() {
    let result = parse_type(
        "struct __cppobj boost::proto::or_<boost::proto::when<boost::proto::binary_expr<boost::proto::tagns_::tag::shift_right,boost::spirit::meta_compiler<boost::spirit::qi::domain>::meta_grammar,boost::spirit::meta_compiler<boost::spirit::qi::domain>::meta_grammar>,boost::spirit::detail::make_binary<boost::spirit::qi::domain,boost::proto::tagns_::tag::shift_right,boost::spirit::meta_compiler<boost::spirit::qi::domain>::meta_grammar,1> >,boost::proto::when<boost::proto::unary_expr<boost::proto::tagns_::tag::shift_right,boost::spirit::meta_compiler<boost::spirit::qi::domain>::meta_grammar>,boost::spirit::detail::make_unary<boost::spirit::qi::domain,boost::proto::tagns_::tag::shift_right,boost::spirit::meta_compiler<boost::spirit::qi::domain>::meta_grammar> >,void,void,void,void,void,void,void,void>::impl<boost::proto::exprns_::expr<boost::proto::tagns_::tag::shift_right,boost::proto::argsns_::list2<boost::proto::exprns_::expr<boost::proto::tagns_::tag::bitwise_or,boost::proto::argsns_::list2<boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::char_code<boost::spirit::tag::alpha,boost::spirit::char_encoding::standard> >,0> &,boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<char const &>,0> >,2> const &,boost::proto::exprns_::expr<boost::proto::tagns_::tag::dereference,boost::proto::argsns_::list1<boost::proto::exprns_::expr<boost::proto::tagns_::tag::bitwise_or,boost::proto::argsns_::list2<boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::char_code<boost::spirit::tag::alnum,boost::spirit::char_encoding::standard> >,0> &,boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<char const &>,0> >,2> const &>,1> const &>,2>,boost::fusion::nil_,boost::spirit::unused_type> : boost::spirit::detail::make_binary<boost::spirit::qi::domain,boost::proto::tagns_::tag::shift_right,boost::spirit::meta_compiler<boost::spirit::qi::domain>::meta_grammar,1>::impl<boost::proto::exprns_::expr<boost::proto::tagns_::tag::shift_right,boost::proto::argsns_::list2<boost::proto::exprns_::expr<boost::proto::tagns_::tag::bitwise_or,boost::proto::argsns_::list2<boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::char_code<boost::spirit::tag::alpha,boost::spirit::char_encoding::standard> >,0> &,boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<char const &>,0> >,2> const &,boost::proto::exprns_::expr<boost::proto::tagns_::tag::dereference,boost::proto::argsns_::list1<boost::proto::exprns_::expr<boost::proto::tagns_::tag::bitwise_or,boost::proto::argsns_::list2<boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::char_code<boost::spirit::tag::alnum,boost::spirit::char_encoding::standard> >,0> &,boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<char const &>,0> >,2> const &>,1> const &>,2>,boost::fusion::nil_,boost::spirit::unused_type>",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "proto",
            "or_<boost::proto::when<boost::proto::binary_expr<boost::proto::tagns_::tag::shift_right,boost::spirit::meta_compiler<boost::spirit::qi::domain>::meta_grammar,boost::spirit::meta_compiler<boost::spirit::qi::domain>::meta_grammar>,boost::spirit::detail::make_binary<boost::spirit::qi::domain,boost::proto::tagns_::tag::shift_right,boost::spirit::meta_compiler<boost::spirit::qi::domain>::meta_grammar,1> >,boost::proto::when<boost::proto::unary_expr<boost::proto::tagns_::tag::shift_right,boost::spirit::meta_compiler<boost::spirit::qi::domain>::meta_grammar>,boost::spirit::detail::make_unary<boost::spirit::qi::domain,boost::proto::tagns_::tag::shift_right,boost::spirit::meta_compiler<boost::spirit::qi::domain>::meta_grammar> >,void,void,void,void,void,void,void,void>",
            "impl<boost::proto::exprns_::expr<boost::proto::tagns_::tag::shift_right,boost::proto::argsns_::list2<boost::proto::exprns_::expr<boost::proto::tagns_::tag::bitwise_or,boost::proto::argsns_::list2<boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::char_code<boost::spirit::tag::alpha,boost::spirit::char_encoding::standard> >,0> &,boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<char const &>,0> >,2> const &,boost::proto::exprns_::expr<boost::proto::tagns_::tag::dereference,boost::proto::argsns_::list1<boost::proto::exprns_::expr<boost::proto::tagns_::tag::bitwise_or,boost::proto::argsns_::list2<boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::char_code<boost::spirit::tag::alnum,boost::spirit::char_encoding::standard> >,0> &,boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<char const &>,0> >,2> const &>,1> const &>,2>,boost::fusion::nil_,boost::spirit::unused_type>"
        ]
    );
}

#[test]
fn test_type_213() {
    let result = parse_type(
        "struct __cppobj boost::proto::reverse_fold<boost::proto::_,boost::proto::make<boost::fusion::nil_>,boost::proto::detail::reverse_fold_tree_<boost::proto::tagns_::tag::shift_right,boost::spirit::detail::make_binary_helper<boost::spirit::meta_compiler<boost::spirit::qi::domain>::meta_grammar> > >::impl<boost::proto::exprns_::expr<boost::proto::tagns_::tag::shift_right,boost::proto::argsns_::list2<boost::spirit::qi::symbols<char,enum dynamo::ast::op_token,boost::spirit::qi::tst<char,enum dynamo::ast::op_token>,boost::spirit::qi::tst_pass_through> &,boost::spirit::qi::rule<char const *,boost::variant<dynamo::ast::nil,float,dynamo::ast::identifier,boost::recursive_wrapper<dynamo::ast::unary_op>,boost::recursive_wrapper<dynamo::ast::binary_op>,boost::recursive_wrapper<dynamo::ast::intrinsic_op>,boost::recursive_wrapper<dynamo::ast::expression>> __cdecl(void),boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::char_code<boost::spirit::tag::space,boost::spirit::char_encoding::ascii> >,0>,boost::spirit::unused_type,boost::spirit::unused_type> &>,2> const &,boost::mpl::void_ const &,boost::spirit::unused_type &> : boost::proto::detail::reverse_fold_impl<boost::proto::make<boost::fusion::nil_>,boost::proto::detail::reverse_fold_tree_<boost::proto::tagns_::tag::shift_right,boost::spirit::detail::make_binary_helper<boost::spirit::meta_compiler<boost::spirit::qi::domain>::meta_grammar> >,boost::proto::exprns_::expr<boost::proto::tagns_::tag::shift_right,boost::proto::argsns_::list2<boost::spirit::qi::symbols<char,enum dynamo::ast::op_token,boost::spirit::qi::tst<char,enum dynamo::ast::op_token>,boost::spirit::qi::tst_pass_through> &,boost::spirit::qi::rule<char const *,boost::variant<dynamo::ast::nil,float,dynamo::ast::identifier,boost::recursive_wrapper<dynamo::ast::unary_op>,boost::recursive_wrapper<dynamo::ast::binary_op>,boost::recursive_wrapper<dynamo::ast::intrinsic_op>,boost::recursive_wrapper<dynamo::ast::expression>> __cdecl(void),boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::char_code<boost::spirit::tag::space,boost::spirit::char_encoding::ascii> >,0>,boost::spirit::unused_type,boost::spirit::unused_type> &>,2> const &,boost::mpl::void_ const &,boost::spirit::unused_type &,2>",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "proto",
            "reverse_fold<boost::proto::_,boost::proto::make<boost::fusion::nil_>,boost::proto::detail::reverse_fold_tree_<boost::proto::tagns_::tag::shift_right,boost::spirit::detail::make_binary_helper<boost::spirit::meta_compiler<boost::spirit::qi::domain>::meta_grammar> > >",
            "impl<boost::proto::exprns_::expr<boost::proto::tagns_::tag::shift_right,boost::proto::argsns_::list2<boost::spirit::qi::symbols<char,enum dynamo::ast::op_token,boost::spirit::qi::tst<char,enum dynamo::ast::op_token>,boost::spirit::qi::tst_pass_through> &,boost::spirit::qi::rule<char const *,boost::variant<dynamo::ast::nil,float,dynamo::ast::identifier,boost::recursive_wrapper<dynamo::ast::unary_op>,boost::recursive_wrapper<dynamo::ast::binary_op>,boost::recursive_wrapper<dynamo::ast::intrinsic_op>,boost::recursive_wrapper<dynamo::ast::expression>> __cdecl(void),boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::char_code<boost::spirit::tag::space,boost::spirit::char_encoding::ascii> >,0>,boost::spirit::unused_type,boost::spirit::unused_type> &>,2> const &,boost::mpl::void_ const &,boost::spirit::unused_type &>"
        ]
    );
}

#[test]
fn test_type_214() {
    let result = parse_type(
        "struct __cppobj boost::proto::switch_<boost::phoenix::meta_grammar,boost::proto::tag_of<boost::proto::_> __cdecl(void)>::impl<boost::proto::exprns_::basic_expr<boost::phoenix::detail::tag::function_eval,boost::proto::argsns_::list3<boost::proto::exprns_::basic_expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<dynamo::annotation<char const *> >,0>,boost::phoenix::actor<boost::spirit::attribute<0> >,boost::phoenix::actor<boost::spirit::argument<0> > >,3> const &,boost::phoenix::vector3<boost::phoenix::actor<boost::proto::exprns_::basic_expr<boost::phoenix::detail::tag::function_eval,boost::proto::argsns_::list3<boost::proto::exprns_::basic_expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<dynamo::annotation<char const *> >,0>,boost::phoenix::actor<boost::spirit::attribute<0> >,boost::phoenix::actor<boost::spirit::argument<0> > >,3> > const *,boost::fusion::vector<char const * &,char const * const &,char const * const &,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_> &,boost::spirit::context<boost::fusion::cons<dynamo::ast::assignment &,boost::fusion::nil_>,boost::fusion::vector0<void> > const &> &,boost::phoenix::default_actions const &> : boost::proto::when<boost::phoenix::detail::rule::function_eval,boost::proto::external_transform>::impl<boost::proto::exprns_::basic_expr<boost::phoenix::detail::tag::function_eval,boost::proto::argsns_::list3<boost::proto::exprns_::basic_expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<dynamo::annotation<char const *> >,0>,boost::phoenix::actor<boost::spirit::attribute<0> >,boost::phoenix::actor<boost::spirit::argument<0> > >,3> const &,boost::phoenix::vector3<boost::phoenix::actor<boost::proto::exprns_::basic_expr<boost::phoenix::detail::tag::function_eval,boost::proto::argsns_::list3<boost::proto::exprns_::basic_expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<dynamo::annotation<char const *> >,0>,boost::phoenix::actor<boost::spirit::attribute<0> >,boost::phoenix::actor<boost::spirit::argument<0> > >,3> > const *,boost::fusion::vector<char const * &,char const * const &,char const * const &,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_> &,boost::spirit::context<boost::fusion::cons<dynamo::ast::assignment &,boost::fusion::nil_>,boost::fusion::vector0<void> > const &> &,boost::phoenix::default_actions const &>",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "proto",
            "switch_<boost::phoenix::meta_grammar,boost::proto::tag_of<boost::proto::_> __cdecl(void)>",
            "impl<boost::proto::exprns_::basic_expr<boost::phoenix::detail::tag::function_eval,boost::proto::argsns_::list3<boost::proto::exprns_::basic_expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<dynamo::annotation<char const *> >,0>,boost::phoenix::actor<boost::spirit::attribute<0> >,boost::phoenix::actor<boost::spirit::argument<0> > >,3> const &,boost::phoenix::vector3<boost::phoenix::actor<boost::proto::exprns_::basic_expr<boost::phoenix::detail::tag::function_eval,boost::proto::argsns_::list3<boost::proto::exprns_::basic_expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<dynamo::annotation<char const *> >,0>,boost::phoenix::actor<boost::spirit::attribute<0> >,boost::phoenix::actor<boost::spirit::argument<0> > >,3> > const *,boost::fusion::vector<char const * &,char const * const &,char const * const &,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_> &,boost::spirit::context<boost::fusion::cons<dynamo::ast::assignment &,boost::fusion::nil_>,boost::fusion::vector0<void> > const &> &,boost::phoenix::default_actions const &>"
        ]
    );
}

#[test]
fn test_type_215() {
    let result = parse_type(
        "struct __cppobj boost::proto::switch_<boost::spirit::meta_compiler<boost::spirit::qi::domain>::cases,boost::proto::tag_of<boost::proto::_> __cdecl(void)>::impl<boost::proto::exprns_::expr<boost::proto::tagns_::tag::shift_right,boost::proto::argsns_::list2<boost::proto::exprns_::expr<boost::proto::tagns_::tag::bitwise_or,boost::proto::argsns_::list2<boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::char_code<boost::spirit::tag::alpha,boost::spirit::char_encoding::standard> >,0> &,boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<char const &>,0> >,2> const &,boost::proto::exprns_::expr<boost::proto::tagns_::tag::dereference,boost::proto::argsns_::list1<boost::proto::exprns_::expr<boost::proto::tagns_::tag::bitwise_or,boost::proto::argsns_::list2<boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::char_code<boost::spirit::tag::alnum,boost::spirit::char_encoding::standard> >,0> &,boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<char const &>,0> >,2> const &>,1> const &>,2> const &,boost::fusion::nil_,boost::spirit::unused_type &> : boost::proto::or_<boost::proto::when<boost::proto::binary_expr<boost::proto::tagns_::tag::shift_right,boost::spirit::meta_compiler<boost::spirit::qi::domain>::meta_grammar,boost::spirit::meta_compiler<boost::spirit::qi::domain>::meta_grammar>,boost::spirit::detail::make_binary<boost::spirit::qi::domain,boost::proto::tagns_::tag::shift_right,boost::spirit::meta_compiler<boost::spirit::qi::domain>::meta_grammar,1> >,boost::proto::when<boost::proto::unary_expr<boost::proto::tagns_::tag::shift_right,boost::spirit::meta_compiler<boost::spirit::qi::domain>::meta_grammar>,boost::spirit::detail::make_unary<boost::spirit::qi::domain,boost::proto::tagns_::tag::shift_right,boost::spirit::meta_compiler<boost::spirit::qi::domain>::meta_grammar> >,void,void,void,void,void,void,void,void>::impl<boost::proto::exprns_::expr<boost::proto::tagns_::tag::shift_right,boost::proto::argsns_::list2<boost::proto::exprns_::expr<boost::proto::tagns_::tag::bitwise_or,boost::proto::argsns_::list2<boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::char_code<boost::spirit::tag::alpha,boost::spirit::char_encoding::standard> >,0> &,boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<char const &>,0> >,2> const &,boost::proto::exprns_::expr<boost::proto::tagns_::tag::dereference,boost::proto::argsns_::list1<boost::proto::exprns_::expr<boost::proto::tagns_::tag::bitwise_or,boost::proto::argsns_::list2<boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::char_code<boost::spirit::tag::alnum,boost::spirit::char_encoding::standard> >,0> &,boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<char const &>,0> >,2> const &>,1> const &>,2> const &,boost::fusion::nil_,boost::spirit::unused_type &>",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "proto",
            "switch_<boost::spirit::meta_compiler<boost::spirit::qi::domain>::cases,boost::proto::tag_of<boost::proto::_> __cdecl(void)>",
            "impl<boost::proto::exprns_::expr<boost::proto::tagns_::tag::shift_right,boost::proto::argsns_::list2<boost::proto::exprns_::expr<boost::proto::tagns_::tag::bitwise_or,boost::proto::argsns_::list2<boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::char_code<boost::spirit::tag::alpha,boost::spirit::char_encoding::standard> >,0> &,boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<char const &>,0> >,2> const &,boost::proto::exprns_::expr<boost::proto::tagns_::tag::dereference,boost::proto::argsns_::list1<boost::proto::exprns_::expr<boost::proto::tagns_::tag::bitwise_or,boost::proto::argsns_::list2<boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::char_code<boost::spirit::tag::alnum,boost::spirit::char_encoding::standard> >,0> &,boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<char const &>,0> >,2> const &>,1> const &>,2> const &,boost::fusion::nil_,boost::spirit::unused_type &>"
        ]
    );
}

#[test]
fn test_type_216() {
    let result = parse_type(
        "struct __cppobj boost::proto::transform_impl<boost::phoenix::actor<boost::proto::exprns_::basic_expr<boost::phoenix::detail::tag::function_eval,boost::proto::argsns_::list4<boost::proto::exprns_::basic_expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<dynamo::error_handler<char const *> >,0>,boost::proto::exprns_::basic_expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<char const *>,0>,boost::phoenix::actor<boost::spirit::argument<3> >,boost::phoenix::actor<boost::spirit::argument<2> > >,4> > const &,boost::phoenix::vector2<boost::phoenix::vector4<boost::phoenix::actor<boost::proto::exprns_::basic_expr<boost::phoenix::detail::tag::function_eval,boost::proto::argsns_::list4<boost::proto::exprns_::basic_expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<dynamo::error_handler<char const *> >,0>,boost::proto::exprns_::basic_expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<char const *>,0>,boost::phoenix::actor<boost::spirit::argument<3> >,boost::phoenix::actor<boost::spirit::argument<2> > >,4> > const *,boost::fusion::vector<char const * &,char const * const &,char const * const &,boost::spirit::info const &,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_> &,boost::spirit::context<boost::fusion::cons<dynamo::ast::expression &,boost::fusion::nil_>,boost::fusion::vector0<void> > &,enum boost::spirit::qi::error_handler_result &> &,boost::phoenix::default_actions const &> const &,boost::proto::envns_::empty_env>",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "proto",
            "transform_impl<boost::phoenix::actor<boost::proto::exprns_::basic_expr<boost::phoenix::detail::tag::function_eval,boost::proto::argsns_::list4<boost::proto::exprns_::basic_expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<dynamo::error_handler<char const *> >,0>,boost::proto::exprns_::basic_expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<char const *>,0>,boost::phoenix::actor<boost::spirit::argument<3> >,boost::phoenix::actor<boost::spirit::argument<2> > >,4> > const &,boost::phoenix::vector2<boost::phoenix::vector4<boost::phoenix::actor<boost::proto::exprns_::basic_expr<boost::phoenix::detail::tag::function_eval,boost::proto::argsns_::list4<boost::proto::exprns_::basic_expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<dynamo::error_handler<char const *> >,0>,boost::proto::exprns_::basic_expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<char const *>,0>,boost::phoenix::actor<boost::spirit::argument<3> >,boost::phoenix::actor<boost::spirit::argument<2> > >,4> > const *,boost::fusion::vector<char const * &,char const * const &,char const * const &,boost::spirit::info const &,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_> &,boost::spirit::context<boost::fusion::cons<dynamo::ast::expression &,boost::fusion::nil_>,boost::fusion::vector0<void> > &,enum boost::spirit::qi::error_handler_result &> &,boost::phoenix::default_actions const &> const &,boost::proto::envns_::empty_env>"
        ]
    );
}

#[test]
fn test_type_217() {
    let result = parse_type(
        "struct __cppobj boost::proto::transform_impl<boost::phoenix::actor<boost::spirit::argument<3> > const &,boost::mpl::bool_<1>,boost::phoenix::is_nullary &>",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "proto",
            "transform_impl<boost::phoenix::actor<boost::spirit::argument<3> > const &,boost::mpl::bool_<1>,boost::phoenix::is_nullary &>"
        ]
    );
}

#[test]
fn test_type_218() {
    let result = parse_type(
        "struct __cppobj boost::proto::transform_impl<boost::proto::exprns_::expr<boost::proto::tagns_::tag::modulus,boost::proto::argsns_::list2<boost::spirit::qi::rule<char const *,dynamo::ast::expression __cdecl(void),boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::char_code<boost::spirit::tag::space,boost::spirit::char_encoding::ascii> >,0>,boost::spirit::unused_type,boost::spirit::unused_type> &,boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<char const &>,0> >,2>,boost::mpl::void_,boost::spirit::unused_type>",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "proto",
            "transform_impl<boost::proto::exprns_::expr<boost::proto::tagns_::tag::modulus,boost::proto::argsns_::list2<boost::spirit::qi::rule<char const *,dynamo::ast::expression __cdecl(void),boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::char_code<boost::spirit::tag::space,boost::spirit::char_encoding::ascii> >,0>,boost::spirit::unused_type,boost::spirit::unused_type> &,boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<char const &>,0> >,2>,boost::mpl::void_,boost::spirit::unused_type>"
        ]
    );
}

#[test]
fn test_type_219() {
    let result = parse_type(
        "struct __cppobj boost::proto::transform_impl<boost::proto::exprns_::expr<boost::proto::tagns_::tag::subscript,boost::proto::argsns_::list2<boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::lexeme>,0> &,boost::proto::exprns_::expr<boost::proto::tagns_::tag::shift_right,boost::proto::argsns_::list2<boost::spirit::qi::symbols<char,enum dynamo::ast::op_token,boost::spirit::qi::tst<char,enum dynamo::ast::op_token>,boost::spirit::qi::tst_pass_through> &,boost::proto::exprns_::expr<boost::proto::tagns_::tag::logical_not,boost::proto::argsns_::list1<boost::proto::exprns_::expr<boost::proto::tagns_::tag::bitwise_or,boost::proto::argsns_::list2<boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::char_code<boost::spirit::tag::alnum,boost::spirit::char_encoding::standard> >,0> &,boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<char const &>,0> >,2> const &>,1> const &>,2> const &>,2> const &,boost::fusion::cons<boost::spirit::qi::raw_directive<boost::spirit::qi::lexeme_directive<boost::spirit::qi::sequence<boost::fusion::cons<boost::spirit::qi::alternative<boost::fusion::cons<boost::spirit::qi::char_class<boost::spirit::tag::char_code<boost::spirit::tag::alpha,boost::spirit::char_encoding::standard> >,boost::fusion::cons<boost::spirit::qi::literal_char<boost::spirit::char_encoding::standard,1,0>,boost::fusion::nil_> > >,boost::fusion::cons<boost::spirit::qi::kleene<boost::spirit::qi::alternative<boost::fusion::cons<boost::spirit::qi::char_class<boost::spirit::tag::char_code<boost::spirit::tag::alnum,boost::spirit::char_encoding::standard> >,boost::fusion::cons<boost::spirit::qi::literal_char<boost::spirit::char_encoding::standard,1,0>,boost::fusion::nil_> > > >,boost::fusion::nil_> > > > >,boost::fusion::nil_>,boost::spirit::unused_type &>",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "proto",
            "transform_impl<boost::proto::exprns_::expr<boost::proto::tagns_::tag::subscript,boost::proto::argsns_::list2<boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::lexeme>,0> &,boost::proto::exprns_::expr<boost::proto::tagns_::tag::shift_right,boost::proto::argsns_::list2<boost::spirit::qi::symbols<char,enum dynamo::ast::op_token,boost::spirit::qi::tst<char,enum dynamo::ast::op_token>,boost::spirit::qi::tst_pass_through> &,boost::proto::exprns_::expr<boost::proto::tagns_::tag::logical_not,boost::proto::argsns_::list1<boost::proto::exprns_::expr<boost::proto::tagns_::tag::bitwise_or,boost::proto::argsns_::list2<boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::char_code<boost::spirit::tag::alnum,boost::spirit::char_encoding::standard> >,0> &,boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<char const &>,0> >,2> const &>,1> const &>,2> const &>,2> const &,boost::fusion::cons<boost::spirit::qi::raw_directive<boost::spirit::qi::lexeme_directive<boost::spirit::qi::sequence<boost::fusion::cons<boost::spirit::qi::alternative<boost::fusion::cons<boost::spirit::qi::char_class<boost::spirit::tag::char_code<boost::spirit::tag::alpha,boost::spirit::char_encoding::standard> >,boost::fusion::cons<boost::spirit::qi::literal_char<boost::spirit::char_encoding::standard,1,0>,boost::fusion::nil_> > >,boost::fusion::cons<boost::spirit::qi::kleene<boost::spirit::qi::alternative<boost::fusion::cons<boost::spirit::qi::char_class<boost::spirit::tag::char_code<boost::spirit::tag::alnum,boost::spirit::char_encoding::standard> >,boost::fusion::cons<boost::spirit::qi::literal_char<boost::spirit::char_encoding::standard,1,0>,boost::fusion::nil_> > > >,boost::fusion::nil_> > > > >,boost::fusion::nil_>,boost::spirit::unused_type &>"
        ]
    );
}

#[test]
fn test_type_220() {
    let result = parse_type(
        "struct __cppobj boost::proto::transform_impl<boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<std::vector<dynamo::ast::expression> (__cdecl&)(void)>,0>,int,int>",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "proto",
            "transform_impl<boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<std::vector<dynamo::ast::expression> (__cdecl&)(void)>,0>,int,int>"
        ]
    );
}

#[test]
fn test_type_221() {
    let result = parse_type(
        "struct __cppobj boost::proto::transform_impl<boost::spirit::qi::rule<char const *,std::vector<dynamo::ast::expression> __cdecl(void),boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::char_code<boost::spirit::tag::space,boost::spirit::char_encoding::ascii> >,0>,boost::spirit::unused_type,boost::spirit::unused_type>,boost::fusion::cons<boost::spirit::qi::literal_char<boost::spirit::char_encoding::standard,1,0>,boost::fusion::nil_>,boost::spirit::unused_type>",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "proto",
            "transform_impl<boost::spirit::qi::rule<char const *,std::vector<dynamo::ast::expression> __cdecl(void),boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::char_code<boost::spirit::tag::space,boost::spirit::char_encoding::ascii> >,0>,boost::spirit::unused_type,boost::spirit::unused_type>,boost::fusion::cons<boost::spirit::qi::literal_char<boost::spirit::char_encoding::standard,1,0>,boost::fusion::nil_>,boost::spirit::unused_type>"
        ]
    );
}

#[test]
fn test_type_222() {
    let result = parse_type(
        "struct __cppobj boost::proto::transform<boost::proto::reverse_fold<boost::proto::_,boost::proto::_state,boost::proto::detail::reverse_fold_tree_<boost::proto::tagns_::tag::bitwise_or,boost::spirit::detail::make_binary_helper<boost::spirit::meta_compiler<boost::spirit::qi::domain>::meta_grammar> > >,void>",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "proto",
            "transform<boost::proto::reverse_fold<boost::proto::_,boost::proto::_state,boost::proto::detail::reverse_fold_tree_<boost::proto::tagns_::tag::bitwise_or,boost::spirit::detail::make_binary_helper<boost::spirit::meta_compiler<boost::spirit::qi::domain>::meta_grammar> > >,void>"
        ]
    );
}

#[test]
fn test_type_223() {
    let result = parse_type(
        "struct __cppobj boost::proto::transform<boost::proto::switch_<boost::spirit::meta_compiler<boost::spirit::qi::domain>::cases,boost::proto::tag_of<boost::proto::_> __cdecl(void)>,void>::result<boost::spirit::meta_compiler<boost::spirit::qi::domain>::meta_grammar __cdecl(boost::proto::exprns_::expr<boost::proto::tagns_::tag::shift_right,boost::proto::argsns_::list2<boost::spirit::qi::rule<char const *,dynamo::ast::identifier __cdecl(void),boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::char_code<boost::spirit::tag::space,boost::spirit::char_encoding::ascii> >,0>,boost::spirit::unused_type,boost::spirit::unused_type> &,boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<char const &>,0> >,2>,boost::fusion::cons<boost::spirit::qi::reference<boost::spirit::qi::rule<char const *,dynamo::ast::expression __cdecl(void),boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::char_code<boost::spirit::tag::space,boost::spirit::char_encoding::ascii> >,0>,boost::spirit::unused_type,boost::spirit::unused_type> const >,boost::fusion::nil_>,boost::spirit::unused_type)>",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "proto",
            "transform<boost::proto::switch_<boost::spirit::meta_compiler<boost::spirit::qi::domain>::cases,boost::proto::tag_of<boost::proto::_> __cdecl(void)>,void>",
            "result<boost::spirit::meta_compiler<boost::spirit::qi::domain>::meta_grammar __cdecl(boost::proto::exprns_::expr<boost::proto::tagns_::tag::shift_right,boost::proto::argsns_::list2<boost::spirit::qi::rule<char const *,dynamo::ast::identifier __cdecl(void),boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::char_code<boost::spirit::tag::space,boost::spirit::char_encoding::ascii> >,0>,boost::spirit::unused_type,boost::spirit::unused_type> &,boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<char const &>,0> >,2>,boost::fusion::cons<boost::spirit::qi::reference<boost::spirit::qi::rule<char const *,dynamo::ast::expression __cdecl(void),boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::char_code<boost::spirit::tag::space,boost::spirit::char_encoding::ascii> >,0>,boost::spirit::unused_type,boost::spirit::unused_type> const >,boost::fusion::nil_>,boost::spirit::unused_type)>"
        ]
    );
}

#[test]
fn test_type_224() {
    let result = parse_type(
        "struct __cppobj boost::proto::when<boost::phoenix::rule::custom_terminal,boost::proto::external_transform>::impl<boost::phoenix::actor<boost::spirit::attribute<0> > const &,boost::phoenix::vector3<boost::phoenix::actor<boost::proto::exprns_::basic_expr<boost::phoenix::detail::tag::function_eval,boost::proto::argsns_::list3<boost::proto::exprns_::basic_expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<dynamo::annotation<char const *> >,0>,boost::phoenix::actor<boost::spirit::attribute<0> >,boost::phoenix::actor<boost::spirit::argument<0> > >,3> > const *,boost::fusion::vector<char const * &,char const * const &,char const * const &,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_> &,boost::spirit::context<boost::fusion::cons<dynamo::ast::assignment &,boost::fusion::nil_>,boost::fusion::vector0<void> > const &> &,boost::phoenix::default_actions const &> : boost::proto::lazy<boost::phoenix::custom_terminal<boost::proto::_value,void> __cdecl(boost::proto::_value,boost::phoenix::_context)>::impl<boost::phoenix::actor<boost::spirit::attribute<0> > const &,boost::phoenix::vector3<boost::phoenix::actor<boost::proto::exprns_::basic_expr<boost::phoenix::detail::tag::function_eval,boost::proto::argsns_::list3<boost::proto::exprns_::basic_expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<dynamo::annotation<char const *> >,0>,boost::phoenix::actor<boost::spirit::attribute<0> >,boost::phoenix::actor<boost::spirit::argument<0> > >,3> > const *,boost::fusion::vector<char const * &,char const * const &,char const * const &,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_> &,boost::spirit::context<boost::fusion::cons<dynamo::ast::assignment &,boost::fusion::nil_>,boost::fusion::vector0<void> > const &> &,boost::phoenix::default_actions const &>",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "proto",
            "when<boost::phoenix::rule::custom_terminal,boost::proto::external_transform>",
            "impl<boost::phoenix::actor<boost::spirit::attribute<0> > const &,boost::phoenix::vector3<boost::phoenix::actor<boost::proto::exprns_::basic_expr<boost::phoenix::detail::tag::function_eval,boost::proto::argsns_::list3<boost::proto::exprns_::basic_expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<dynamo::annotation<char const *> >,0>,boost::phoenix::actor<boost::spirit::attribute<0> >,boost::phoenix::actor<boost::spirit::argument<0> > >,3> > const *,boost::fusion::vector<char const * &,char const * const &,char const * const &,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_> &,boost::spirit::context<boost::fusion::cons<dynamo::ast::assignment &,boost::fusion::nil_>,boost::fusion::vector0<void> > const &> &,boost::phoenix::default_actions const &>"
        ]
    );
}

#[test]
fn test_type_225() {
    let result = parse_type(
        "struct __cppobj boost::rational<int> : boost::less_than_comparable<boost::rational<int>,boost::equality_comparable<boost::rational<int>,boost::less_than_comparable2<boost::rational<int>,int,boost::equality_comparable2<boost::rational<int>,int,boost::addable<boost::rational<int>,boost::subtractable<boost::rational<int>,boost::multipliable<boost::rational<int>,boost::dividable<boost::rational<int>,boost::addable2<boost::rational<int>,int,boost::subtractable2<boost::rational<int>,int,boost::subtractable2_left<boost::rational<int>,int,boost::multipliable2<boost::rational<int>,int,boost::dividable2<boost::rational<int>,int,boost::dividable2_left<boost::rational<int>,int,boost::incrementable<boost::rational<int>,boost::decrementable<boost::rational<int>,boost::detail::empty_base<boost::rational<int> > > > > > > > > >,boost::detail::empty_base<boost::rational<int> >,boost::detail::true_t>,boost::detail::empty_base<boost::rational<int> >,boost::detail::true_t>,boost::detail::empty_base<boost::rational<int> >,boost::detail::true_t>,boost::detail::empty_base<boost::rational<int> >,boost::detail::true_t> > >,boost::detail::empty_base<boost::rational<int> >,boost::detail::true_t>,boost::detail::empty_base<boost::rational<int> >,boost::detail::true_t>",
    );
    assert_eq!(result, vec!["boost", "rational<int>"]);
}

#[test]
fn test_type_226() {
    let result = parse_type("struct __cppobj boost::shared_ptr<CEffectsTemplate>");
    assert_eq!(result, vec!["boost", "shared_ptr<CEffectsTemplate>"]);
}

#[test]
fn test_type_227() {
    let result = parse_type("struct __cppobj boost::shared_ptr<CGrenade>");
    assert_eq!(result, vec!["boost", "shared_ptr<CGrenade>"]);
}

#[test]
fn test_type_228() {
    let result = parse_type(
        "struct __cppobj boost::shared_ptr<TaskQueue::detail::CAsyncWork<A0x743c4eea::<lambda218>,A0x743c4eea::SJobResult,void> >",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "shared_ptr<TaskQueue::detail::CAsyncWork<A0x743c4eea::<lambda218>,A0x743c4eea::SJobResult,void> >"
        ]
    );
}

#[test]
fn test_type_229() {
    let result = parse_type(
        "struct __cppobj boost::spirit::argument<2>::result<boost::phoenix::vector4<boost::phoenix::actor<boost::proto::exprns_::basic_expr<boost::phoenix::detail::tag::function_eval,boost::proto::argsns_::list4<boost::proto::exprns_::basic_expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<dynamo::error_handler<char const *> >,0>,boost::proto::exprns_::basic_expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<char const *>,0>,boost::phoenix::actor<boost::spirit::argument<3> >,boost::phoenix::actor<boost::spirit::argument<2> > >,4> > const *,boost::fusion::vector<char const * &,char const * const &,char const * const &,boost::spirit::info const &,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_> const &,boost::spirit::context<boost::fusion::cons<dynamo::ast::expression &,boost::fusion::nil_>,boost::fusion::vector0<void> > const &,enum boost::spirit::qi::error_handler_result const &> >",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "spirit",
            "argument<2>",
            "result<boost::phoenix::vector4<boost::phoenix::actor<boost::proto::exprns_::basic_expr<boost::phoenix::detail::tag::function_eval,boost::proto::argsns_::list4<boost::proto::exprns_::basic_expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<dynamo::error_handler<char const *> >,0>,boost::proto::exprns_::basic_expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<char const *>,0>,boost::phoenix::actor<boost::spirit::argument<3> >,boost::phoenix::actor<boost::spirit::argument<2> > >,4> > const *,boost::fusion::vector<char const * &,char const * const &,char const * const &,boost::spirit::info const &,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_> const &,boost::spirit::context<boost::fusion::cons<dynamo::ast::expression &,boost::fusion::nil_>,boost::fusion::vector0<void> > const &,enum boost::spirit::qi::error_handler_result const &> >"
        ]
    );
}

#[test]
fn test_type_230() {
    let result = parse_type(
        "struct __cppobj boost::spirit::detail::make_binary_helper<boost::spirit::meta_compiler<boost::spirit::qi::domain>::meta_grammar>::impl<boost::proto::exprns_::expr<boost::proto::tagns_::tag::logical_not,boost::proto::argsns_::list1<boost::proto::exprns_::expr<boost::proto::tagns_::tag::bitwise_or,boost::proto::argsns_::list2<boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::char_code<boost::spirit::tag::alnum,boost::spirit::char_encoding::standard> >,0> &,boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<char const &>,0> >,2> const &>,1>,boost::fusion::nil_,boost::spirit::unused_type> : boost::proto::transform_impl<boost::proto::exprns_::expr<boost::proto::tagns_::tag::logical_not,boost::proto::argsns_::list1<boost::proto::exprns_::expr<boost::proto::tagns_::tag::bitwise_or,boost::proto::argsns_::list2<boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::char_code<boost::spirit::tag::alnum,boost::spirit::char_encoding::standard> >,0> &,boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<char const &>,0> >,2> const &>,1>,boost::fusion::nil_,boost::spirit::unused_type>",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "spirit",
            "detail",
            "make_binary_helper<boost::spirit::meta_compiler<boost::spirit::qi::domain>::meta_grammar>",
            "impl<boost::proto::exprns_::expr<boost::proto::tagns_::tag::logical_not,boost::proto::argsns_::list1<boost::proto::exprns_::expr<boost::proto::tagns_::tag::bitwise_or,boost::proto::argsns_::list2<boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::char_code<boost::spirit::tag::alnum,boost::spirit::char_encoding::standard> >,0> &,boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<char const &>,0> >,2> const &>,1>,boost::fusion::nil_,boost::spirit::unused_type>"
        ]
    );
}

#[test]
fn test_type_231() {
    let result = parse_type(
        "struct __cppobj boost::spirit::detail::make_binary<boost::spirit::qi::domain,boost::proto::tagns_::tag::bitwise_or,boost::spirit::meta_compiler<boost::spirit::qi::domain>::meta_grammar,1>::impl<boost::proto::exprns_::expr<boost::proto::tagns_::tag::bitwise_or,boost::proto::argsns_::list2<boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::char_code<boost::spirit::tag::alpha,boost::spirit::char_encoding::standard> >,0> &,boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<char const &>,0> >,2> const &,boost::fusion::cons<boost::spirit::qi::kleene<boost::spirit::qi::alternative<boost::fusion::cons<boost::spirit::qi::char_class<boost::spirit::tag::char_code<boost::spirit::tag::alnum,boost::spirit::char_encoding::standard> >,boost::fusion::cons<boost::spirit::qi::literal_char<boost::spirit::char_encoding::standard,1,0>,boost::fusion::nil_> > > >,boost::fusion::nil_> const &,boost::spirit::unused_type &> : boost::proto::transform_impl<boost::proto::exprns_::expr<boost::proto::tagns_::tag::bitwise_or,boost::proto::argsns_::list2<boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::char_code<boost::spirit::tag::alpha,boost::spirit::char_encoding::standard> >,0> &,boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<char const &>,0> >,2> const &,boost::fusion::cons<boost::spirit::qi::kleene<boost::spirit::qi::alternative<boost::fusion::cons<boost::spirit::qi::char_class<boost::spirit::tag::char_code<boost::spirit::tag::alnum,boost::spirit::char_encoding::standard> >,boost::fusion::cons<boost::spirit::qi::literal_char<boost::spirit::char_encoding::standard,1,0>,boost::fusion::nil_> > > >,boost::fusion::nil_> const &,boost::spirit::unused_type &>",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "spirit",
            "detail",
            "make_binary<boost::spirit::qi::domain,boost::proto::tagns_::tag::bitwise_or,boost::spirit::meta_compiler<boost::spirit::qi::domain>::meta_grammar,1>",
            "impl<boost::proto::exprns_::expr<boost::proto::tagns_::tag::bitwise_or,boost::proto::argsns_::list2<boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::char_code<boost::spirit::tag::alpha,boost::spirit::char_encoding::standard> >,0> &,boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<char const &>,0> >,2> const &,boost::fusion::cons<boost::spirit::qi::kleene<boost::spirit::qi::alternative<boost::fusion::cons<boost::spirit::qi::char_class<boost::spirit::tag::char_code<boost::spirit::tag::alnum,boost::spirit::char_encoding::standard> >,boost::fusion::cons<boost::spirit::qi::literal_char<boost::spirit::char_encoding::standard,1,0>,boost::fusion::nil_> > > >,boost::fusion::nil_> const &,boost::spirit::unused_type &>"
        ]
    );
}

#[test]
fn test_type_232() {
    let result = parse_type(
        "struct __cppobj boost::spirit::detail::make_terminal<boost::spirit::qi::domain>::impl<boost::spirit::qi::symbols<char,enum dynamo::ast::op_token,boost::spirit::qi::tst<char,enum dynamo::ast::op_token>,boost::spirit::qi::tst_pass_through> &,boost::fusion::cons<boost::spirit::qi::literal_char<boost::spirit::char_encoding::standard,1,0>,boost::fusion::nil_>,boost::spirit::unused_type &> : boost::spirit::detail::make_terminal_impl<boost::spirit::qi::symbols<char,enum dynamo::ast::op_token,boost::spirit::qi::tst<char,enum dynamo::ast::op_token>,boost::spirit::qi::tst_pass_through> &,boost::fusion::cons<boost::spirit::qi::literal_char<boost::spirit::char_encoding::standard,1,0>,boost::fusion::nil_>,boost::spirit::unused_type &,boost::spirit::qi::domain>",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "spirit",
            "detail",
            "make_terminal<boost::spirit::qi::domain>",
            "impl<boost::spirit::qi::symbols<char,enum dynamo::ast::op_token,boost::spirit::qi::tst<char,enum dynamo::ast::op_token>,boost::spirit::qi::tst_pass_through> &,boost::fusion::cons<boost::spirit::qi::literal_char<boost::spirit::char_encoding::standard,1,0>,boost::fusion::nil_>,boost::spirit::unused_type &>"
        ]
    );
}

#[test]
fn test_type_233() {
    let result = parse_type(
        "struct __cppobj boost::spirit::make_component<boost::spirit::qi::domain,boost::proto::tagns_::tag::unary_plus,void>::result<boost::spirit::make_component<boost::spirit::qi::domain,boost::proto::tagns_::tag::unary_plus,void> __cdecl(boost::fusion::cons<boost::spirit::qi::reference<boost::spirit::qi::rule<char const *,boost::variant<dynamo::ast::nil,dynamo::ast::assignment,dynamo::ast::expression> __cdecl(void),boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::char_code<boost::spirit::tag::space,boost::spirit::char_encoding::ascii> >,0>,boost::spirit::unused_type,boost::spirit::unused_type> const >,boost::fusion::nil_>,boost::spirit::unused_type &)>",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "spirit",
            "make_component<boost::spirit::qi::domain,boost::proto::tagns_::tag::unary_plus,void>",
            "result<boost::spirit::make_component<boost::spirit::qi::domain,boost::proto::tagns_::tag::unary_plus,void> __cdecl(boost::fusion::cons<boost::spirit::qi::reference<boost::spirit::qi::rule<char const *,boost::variant<dynamo::ast::nil,dynamo::ast::assignment,dynamo::ast::expression> __cdecl(void),boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::char_code<boost::spirit::tag::space,boost::spirit::char_encoding::ascii> >,0>,boost::spirit::unused_type,boost::spirit::unused_type> const >,boost::fusion::nil_>,boost::spirit::unused_type &)>"
        ]
    );
}

#[test]
fn test_type_234() {
    let result = parse_type(
        "struct __cppobj boost::spirit::make_component<boost::spirit::qi::domain,boost::spirit::tag::directive,void>",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "spirit",
            "make_component<boost::spirit::qi::domain,boost::spirit::tag::directive,void>"
        ]
    );
}

#[test]
fn test_type_235() {
    let result = parse_type(
        "struct __cppobj boost::spirit::qi::detail::pass_container<boost::spirit::qi::detail::fail_function<char const *,boost::spirit::context<boost::fusion::cons<dynamo::ast::expression &,boost::fusion::nil_>,boost::fusion::vector0<void> >,boost::spirit::qi::char_class<boost::spirit::tag::char_code<boost::spirit::tag::space,boost::spirit::char_encoding::ascii> > >,std::vector<dynamo::ast::binary_op>,boost::mpl::bool_<0> >",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "spirit",
            "qi",
            "detail",
            "pass_container<boost::spirit::qi::detail::fail_function<char const *,boost::spirit::context<boost::fusion::cons<dynamo::ast::expression &,boost::fusion::nil_>,boost::fusion::vector0<void> >,boost::spirit::qi::char_class<boost::spirit::tag::char_code<boost::spirit::tag::space,boost::spirit::char_encoding::ascii> > >,std::vector<dynamo::ast::binary_op>,boost::mpl::bool_<0> >"
        ]
    );
}

#[test]
fn test_type_236() {
    let result = parse_type(
        "struct __cppobj boost::spirit::qi::make_composite<boost::proto::tagns_::tag::unary_plus,boost::fusion::cons<boost::spirit::qi::reference<boost::spirit::qi::rule<char const *,boost::variant<dynamo::ast::nil,dynamo::ast::assignment,dynamo::ast::expression> __cdecl(void),boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::char_code<boost::spirit::tag::space,boost::spirit::char_encoding::ascii> >,0>,boost::spirit::unused_type,boost::spirit::unused_type> const >,boost::fusion::nil_>,boost::spirit::unused_type,void> : boost::spirit::make_unary_composite<boost::fusion::cons<boost::spirit::qi::reference<boost::spirit::qi::rule<char const *,boost::variant<dynamo::ast::nil,dynamo::ast::assignment,dynamo::ast::expression> __cdecl(void),boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::char_code<boost::spirit::tag::space,boost::spirit::char_encoding::ascii> >,0>,boost::spirit::unused_type,boost::spirit::unused_type> const >,boost::fusion::nil_>,boost::spirit::qi::plus>",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "spirit",
            "qi",
            "make_composite<boost::proto::tagns_::tag::unary_plus,boost::fusion::cons<boost::spirit::qi::reference<boost::spirit::qi::rule<char const *,boost::variant<dynamo::ast::nil,dynamo::ast::assignment,dynamo::ast::expression> __cdecl(void),boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::char_code<boost::spirit::tag::space,boost::spirit::char_encoding::ascii> >,0>,boost::spirit::unused_type,boost::spirit::unused_type> const >,boost::fusion::nil_>,boost::spirit::unused_type,void>"
        ]
    );
}

#[test]
fn test_type_237() {
    let result = parse_type(
        "struct __cppobj boost::spirit::qi::rule<char const *,dynamo::ast::intrinsic_op __cdecl(void),boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::char_code<boost::spirit::tag::space,boost::spirit::char_encoding::ascii> >,0>,boost::spirit::unused_type,boost::spirit::unused_type>::attribute<boost::spirit::context<boost::fusion::cons<boost::variant<dynamo::ast::nil,float,dynamo::ast::identifier,boost::recursive_wrapper<dynamo::ast::unary_op>,boost::recursive_wrapper<dynamo::ast::binary_op>,boost::recursive_wrapper<dynamo::ast::intrinsic_op>,boost::recursive_wrapper<dynamo::ast::expression>> &,boost::fusion::nil_>,boost::fusion::vector0<void> >,char const *>",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "spirit",
            "qi",
            "rule<char const *,dynamo::ast::intrinsic_op __cdecl(void),boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::char_code<boost::spirit::tag::space,boost::spirit::char_encoding::ascii> >,0>,boost::spirit::unused_type,boost::spirit::unused_type>",
            "attribute<boost::spirit::context<boost::fusion::cons<boost::variant<dynamo::ast::nil,float,dynamo::ast::identifier,boost::recursive_wrapper<dynamo::ast::unary_op>,boost::recursive_wrapper<dynamo::ast::binary_op>,boost::recursive_wrapper<dynamo::ast::intrinsic_op>,boost::recursive_wrapper<dynamo::ast::expression>> &,boost::fusion::nil_>,boost::fusion::vector0<void> >,char const *>"
        ]
    );
}

#[test]
fn test_type_238() {
    let result = parse_type(
        "struct __cppobj boost::spirit::traits::ischar<char,boost::spirit::char_encoding::standard,0>",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "spirit",
            "traits",
            "ischar<char,boost::spirit::char_encoding::standard,0>"
        ]
    );
}

#[test]
fn test_type_239() {
    let result = parse_type(
        "struct __cppobj boost::unordered::detail::allocator_traits<shared_node_allocator<boost::unordered::detail::ptr_node<std::pair<CStatisticContext const ,int> > > >::pointer_to_other<boost::unordered::detail::ptr_node<std::pair<CStatisticContext const ,int> > const > : boost::pointer_to_other<boost::unordered::detail::ptr_node<std::pair<CStatisticContext const ,int> > *,boost::unordered::detail::ptr_node<std::pair<CStatisticContext const ,int> > const >",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "unordered",
            "detail",
            "allocator_traits<shared_node_allocator<boost::unordered::detail::ptr_node<std::pair<CStatisticContext const ,int> > > >",
            "pointer_to_other<boost::unordered::detail::ptr_node<std::pair<CStatisticContext const ,int> > const >"
        ]
    );
}

#[test]
fn test_type_240() {
    let result = parse_type(
        "struct __cppobj boost::unordered::detail::compressed_base<std::equal_to<std::string >,2> : std::equal_to<std::string >",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "unordered",
            "detail",
            "compressed_base<std::equal_to<std::string >,2>"
        ]
    );
}

#[test]
fn test_type_241() {
    let result = parse_type(
        "struct __cppobj boost::unordered::detail::compressed<SEventIdHasher,std::equal_to<SEventID> > : boost::unordered::detail::compressed_base<SEventIdHasher,1>, boost::unordered::detail::compressed_base<std::equal_to<SEventID>,2>",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "unordered",
            "detail",
            "compressed<SEventIdHasher,std::equal_to<SEventID> >"
        ]
    );
}

#[test]
fn test_type_242() {
    let result =
        parse_type("struct __cppobj boost::unsupported_thread_option : boost::thread_exception");
    assert_eq!(result, vec!["boost", "unsupported_thread_option"]);
}

#[test]
fn test_type_243() {
    let result = parse_type("struct __cppobj boost::uuids::detail::seed_rng");
    assert_eq!(result, vec!["boost", "uuids", "detail", "seed_rng"]);
}

#[test]
fn test_type_244() {
    let result = parse_type("struct __cppobj CAiBTEventHandler : NEvent::CEventHandler");
    assert_eq!(result, vec!["CAiBTEventHandler"]);
}

#[test]
fn test_type_245() {
    let result = parse_type("struct __cppobj CAIRoadObstacle : CGameObject");
    assert_eq!(result, vec!["CAIRoadObstacle"]);
}

#[test]
fn test_type_246() {
    let result =
        parse_type("struct __cppobj CAppSystemCreator<CVocalsManager> : Base::IAppSystemCreator");
    assert_eq!(result, vec!["CAppSystemCreator<CVocalsManager>"]);
}

#[test]
fn test_type_247() {
    let result = parse_type("struct __cppobj CCharacterUpgrade::SUpgradeData");
    assert_eq!(result, vec!["CCharacterUpgrade", "SUpgradeData"]);
}

#[test]
fn test_type_248() {
    let result = parse_type("struct __cppobj CCustomCharacterRBListener::SLedgePoint");
    assert_eq!(result, vec!["CCustomCharacterRBListener", "SLedgePoint"]);
}

#[test]
fn test_type_249() {
    let result = parse_type(
        "struct __cppobj CDamageListenerSystem : Base::IAppSystem, Base::CSingle<CDamageListenerSystem>",
    );
    assert_eq!(result, vec!["CDamageListenerSystem"]);
}

#[test]
fn test_type_250() {
    let result = parse_type("struct __cppobj CFootpathFilterFtor : IRoadFilterFtor");
    assert_eq!(result, vec!["CFootpathFilterFtor"]);
}

#[test]
fn test_type_251() {
    let result =
        parse_type("struct __cppobj CGameObjectCreator<CAIRoadGraphFilter> : IGameObjectCreator");
    assert_eq!(result, vec!["CGameObjectCreator<CAIRoadGraphFilter>"]);
}

#[test]
fn test_type_252() {
    let result = parse_type(
        "struct __cppobj CGameObjectCreator<CDynamicNavMeshCutter> : IGameObjectCreator",
    );
    assert_eq!(result, vec!["CGameObjectCreator<CDynamicNavMeshCutter>"]);
}

#[test]
fn test_type_253() {
    let result =
        parse_type("struct __cppobj CGameObjectCreator<CEffectUIEmitter> : IGameObjectCreator");
    assert_eq!(result, vec!["CGameObjectCreator<CEffectUIEmitter>"]);
}

#[test]
fn test_type_254() {
    let result =
        parse_type("struct __cppobj CGameObjectCreator<CScrapyardROM> : IGameObjectCreator");
    assert_eq!(result, vec!["CGameObjectCreator<CScrapyardROM>"]);
}

#[test]
fn test_type_255() {
    let result =
        parse_type("struct __cppobj CGameObjectCreator<CSniperWeapon>::SGameObjectDestroyer");
    assert_eq!(
        result,
        vec!["CGameObjectCreator<CSniperWeapon>", "SGameObjectDestroyer"]
    );
}

#[test]
fn test_type_256() {
    let result = parse_type("struct __cppobj CHashCache");
    assert_eq!(result, vec!["CHashCache"]);
}

#[test]
fn test_type_257() {
    let result = parse_type("struct __cppobj CIsPlayerNonCombat : NAnimationSystem::CCondition");
    assert_eq!(result, vec!["CIsPlayerNonCombat"]);
}

#[test]
fn test_type_258() {
    let result = parse_type("struct __cppobj CMatchmakingProvider");
    assert_eq!(result, vec!["CMatchmakingProvider"]);
}

#[test]
fn test_type_259() {
    let result =
        parse_type("struct __cppobj COnlineObserver<CAsynchronousBragThresholdObserver<__int64> >");
    assert_eq!(
        result,
        vec!["COnlineObserver<CAsynchronousBragThresholdObserver<__int64> >"]
    );
}

#[test]
fn test_type_260() {
    let result =
        parse_type("struct __cppobj COnlineObserver<CSquareEnixMembershipObserver>::null_deleter");
    assert_eq!(
        result,
        vec![
            "COnlineObserver<CSquareEnixMembershipObserver>",
            "null_deleter"
        ]
    );
}

#[test]
fn test_type_261() {
    let result = parse_type("struct __cppobj CPfxBreakable : CPfxRigidBody, IPartInstanceListener");
    assert_eq!(result, vec!["CPfxBreakable"]);
}

#[test]
fn test_type_262() {
    let result = parse_type("struct __cppobj CrashReporter::ScopedChangeCurrentDirectory");
    assert_eq!(
        result,
        vec!["CrashReporter", "ScopedChangeCurrentDirectory"]
    );
}

#[test]
fn test_type_263() {
    let result = parse_type("struct __cppobj CRawBitmap<_PixelFloatRGBA>");
    assert_eq!(result, vec!["CRawBitmap<_PixelFloatRGBA>"]);
}

#[test]
fn test_type_264() {
    let result = parse_type("struct __cppobj CScrapyardROMNetworkComponent : CROMNetworkComponent");
    assert_eq!(result, vec!["CScrapyardROMNetworkComponent"]);
}

#[test]
fn test_type_265() {
    let result = parse_type("struct __cppobj CSequenceObject2::SSeqTransformTrack");
    assert_eq!(result, vec!["CSequenceObject2", "SSeqTransformTrack"]);
}

#[test]
fn test_type_266() {
    let result = parse_type("struct __cppobj CTankPositionModifier : CCameraModifier");
    assert_eq!(result, vec!["CTankPositionModifier"]);
}

#[test]
fn test_type_267() {
    let result = parse_type("struct __cppobj CWeather::SWeatherParam");
    assert_eq!(result, vec!["CWeather", "SWeatherParam"]);
}

#[test]
fn test_type_268() {
    let result = parse_type("struct __cppobj dynamo::vm::machine");
    assert_eq!(result, vec!["dynamo", "vm", "machine"]);
}

#[test]
fn test_type_269() {
    let result = parse_type("struct __cppobj erase_swap_multiple_Test : testing::Test");
    assert_eq!(result, vec!["erase_swap_multiple_Test"]);
}

#[test]
fn test_type_270() {
    let result = parse_type("struct __cppobj ExtractValueHelper<VehicleValueCache,1644820137>");
    assert_eq!(
        result,
        vec!["ExtractValueHelper<VehicleValueCache,1644820137>"]
    );
}

#[test]
fn test_type_271() {
    let result = parse_type("struct __cppobj ExtractValueHelper<VehicleValueCache,3031391367>");
    assert_eq!(
        result,
        vec!["ExtractValueHelper<VehicleValueCache,3031391367>"]
    );
}

#[test]
fn test_type_272() {
    let result = parse_type("struct __cppobj ExtractValueHelper<VehicleValueCache,3947397009>");
    assert_eq!(
        result,
        vec!["ExtractValueHelper<VehicleValueCache,3947397009>"]
    );
}

#[test]
fn test_type_273() {
    let result = parse_type("struct __cppobj ExtractValueHelper<VehicleValueCache,423992487>");
    assert_eq!(
        result,
        vec!["ExtractValueHelper<VehicleValueCache,423992487>"]
    );
}

#[test]
fn test_type_274() {
    let result = parse_type("struct __cppobj FileInfo");
    assert_eq!(result, vec!["FileInfo"]);
}

#[test]
fn test_type_275() {
    let result = parse_type("struct __cppobj FollowLinkHelper<VehicleValueCache,7>");
    assert_eq!(result, vec!["FollowLinkHelper<VehicleValueCache,7>"]);
}

#[test]
fn test_type_276() {
    let result =
        parse_type("struct __cppobj GeometryCastSegmentQuery : hkcdAabbTreeQueries::AabbCollector");
    assert_eq!(result, vec!["GeometryCastSegmentQuery"]);
}

#[test]
fn test_type_277() {
    let result = parse_type("struct __cppobj Graphics::SCreateSamplerStateParams");
    assert_eq!(result, vec!["Graphics", "SCreateSamplerStateParams"]);
}

#[test]
fn test_type_278() {
    let result =
        parse_type("struct __cppobj hkaiNavMeshGenerationUtils::DeprecatedGenerationOutputs");
    assert_eq!(
        result,
        vec!["hkaiNavMeshGenerationUtils", "DeprecatedGenerationOutputs"]
    );
}

#[test]
fn test_type_279() {
    let result = parse_type(
        "struct __cppobj hkAlgorithm::eq<hkHandle<unsigned int,0,hkcdPlanarGeometryPolygonCollection::PolygonIdDiscriminant> >",
    );
    assert_eq!(
        result,
        vec![
            "hkAlgorithm",
            "eq<hkHandle<unsigned int,0,hkcdPlanarGeometryPolygonCollection::PolygonIdDiscriminant> >"
        ]
    );
}

#[test]
fn test_type_280() {
    let result = parse_type(
        "struct __cppobj hkArray<hkAabb,hkContainerHeapAllocator> : hkArrayBase<hkAabb>",
    );
    assert_eq!(result, vec!["hkArray<hkAabb,hkContainerHeapAllocator>"]);
}

#[test]
fn test_type_281() {
    let result = parse_type(
        "struct __cppobj hkArray<hkArray<hkaiHierarchyUtils::SemiSparse2dArraySorted::Cost,hkContainerHeapAllocator>,hkContainerHeapAllocator> : hkArrayBase<hkArray<hkaiHierarchyUtils::SemiSparse2dArraySorted::Cost,hkContainerHeapAllocator> >",
    );
    assert_eq!(
        result,
        vec![
            "hkArray<hkArray<hkaiHierarchyUtils::SemiSparse2dArraySorted::Cost,hkContainerHeapAllocator>,hkContainerHeapAllocator>"
        ]
    );
}

#[test]
fn test_type_282() {
    let result = parse_type(
        "struct __cppobj hkArray<hkArray<hkHandle<unsigned int,268435455,hkcdPlanarGeometryPrimitives::PlaneIdDiscriminant>,hkContainerHeapAllocator>,hkContainerHeapAllocator> : hkArrayBase<hkArray<hkHandle<unsigned int,268435455,hkcdPlanarGeometryPrimitives::PlaneIdDiscriminant>,hkContainerHeapAllocator> >",
    );
    assert_eq!(
        result,
        vec![
            "hkArray<hkArray<hkHandle<unsigned int,268435455,hkcdPlanarGeometryPrimitives::PlaneIdDiscriminant>,hkContainerHeapAllocator>,hkContainerHeapAllocator>"
        ]
    );
}

#[test]
fn test_type_283() {
    let result = parse_type(
        "struct __cppobj hkArray<hkDisplayGeometry *,hkContainerHeapAllocator> : hkArrayBase<hkDisplayGeometry *>",
    );
    assert_eq!(
        result,
        vec!["hkArray<hkDisplayGeometry *,hkContainerHeapAllocator>"]
    );
}

#[test]
fn test_type_284() {
    let result = parse_type(
        "struct __cppobj hkArray<hkdMeshGraphicsShape::SkinData,hkContainerHeapAllocator> : hkArrayBase<hkdMeshGraphicsShape::SkinData>",
    );
    assert_eq!(
        result,
        vec!["hkArray<hkdMeshGraphicsShape::SkinData,hkContainerHeapAllocator>"]
    );
}

#[test]
fn test_type_285() {
    let result = parse_type(
        "struct __cppobj hkArray<hkMeshShape *,hkContainerHeapAllocator> : hkArrayBase<hkMeshShape *>",
    );
    assert_eq!(
        result,
        vec!["hkArray<hkMeshShape *,hkContainerHeapAllocator>"]
    );
}

#[test]
fn test_type_286() {
    let result = parse_type(
        "struct __cppobj hkArray<hkMultipleVertexBuffer::VertexBufferInfo,hkContainerHeapAllocator> : hkArrayBase<hkMultipleVertexBuffer::VertexBufferInfo>",
    );
    assert_eq!(
        result,
        vec!["hkArray<hkMultipleVertexBuffer::VertexBufferInfo,hkContainerHeapAllocator>"]
    );
}

#[test]
fn test_type_287() {
    let result = parse_type(
        "struct __cppobj hkArray<hknpDeactivatedIsland *,hkContainerHeapAllocator> : hkArrayBase<hknpDeactivatedIsland *>",
    );
    assert_eq!(
        result,
        vec!["hkArray<hknpDeactivatedIsland *,hkContainerHeapAllocator>"]
    );
}

#[test]
fn test_type_288() {
    let result = parse_type(
        "struct __cppobj hkArray<hknpHybridAabbTree<unsigned int,unsigned int,hkAabb16>::Node *,hkContainerHeapAllocator> : hkArrayBase<hknpHybridAabbTree<unsigned int,unsigned int,hkAabb16>::Node *>",
    );
    assert_eq!(
        result,
        vec![
            "hkArray<hknpHybridAabbTree<unsigned int,unsigned int,hkAabb16>::Node *,hkContainerHeapAllocator>"
        ]
    );
}

#[test]
fn test_type_289() {
    let result = parse_type(
        "struct __cppobj hkArray<hkRefPtr<hkaAnimation>,hkContainerHeapAllocator> : hkArrayBase<hkRefPtr<hkaAnimation> >",
    );
    assert_eq!(
        result,
        vec!["hkArray<hkRefPtr<hkaAnimation>,hkContainerHeapAllocator>"]
    );
}

#[test]
fn test_type_290() {
    let result = parse_type(
        "struct __cppobj hkArray<hkxMaterial *,hkContainerHeapAllocator> : hkArrayBase<hkxMaterial *>",
    );
    assert_eq!(
        result,
        vec!["hkArray<hkxMaterial *,hkContainerHeapAllocator>"]
    );
}

#[test]
fn test_type_291() {
    let result = parse_type(
        "struct __cppobj hkArray<LongestEdge,hkContainerTempAllocator> : hkArrayBase<LongestEdge>",
    );
    assert_eq!(
        result,
        vec!["hkArray<LongestEdge,hkContainerTempAllocator>"]
    );
}

#[test]
fn test_type_292() {
    let result = parse_type("struct __cppobj hkArrayBase<hkaiDirectedGraphExplicitCost::Edge>");
    assert_eq!(
        result,
        vec!["hkArrayBase<hkaiDirectedGraphExplicitCost::Edge>"]
    );
}

#[test]
fn test_type_293() {
    let result = parse_type("struct __cppobj hkArrayBase<hkaiNavMeshErosion::Util::EdgeSplit>");
    assert_eq!(
        result,
        vec!["hkArrayBase<hkaiNavMeshErosion::Util::EdgeSplit>"]
    );
}

#[test]
fn test_type_294() {
    let result = parse_type(
        "struct __cppobj hkArrayBase<hkcdDynamicTree::Tree<hkcdDynamicTree::DynamicStorage16>::SAHBin>",
    );
    assert_eq!(
        result,
        vec!["hkArrayBase<hkcdDynamicTree::Tree<hkcdDynamicTree::DynamicStorage16>::SAHBin>"]
    );
}

#[test]
fn test_type_295() {
    let result = parse_type("struct __cppobj hkArrayBase<hkcdPlanarGeometryPrimitives::Plane>");
    assert_eq!(
        result,
        vec!["hkArrayBase<hkcdPlanarGeometryPrimitives::Plane>"]
    );
}

#[test]
fn test_type_296() {
    let result = parse_type("struct __cppobj hkArrayBase<hkcdVoronoiDiagramImpl::VtxCell>");
    assert_eq!(result, vec!["hkArrayBase<hkcdVoronoiDiagramImpl::VtxCell>"]);
}

#[test]
fn test_type_297() {
    let result = parse_type("struct __cppobj hkArrayBase<hkDataClassDict::Enum *>");
    assert_eq!(result, vec!["hkArrayBase<hkDataClassDict::Enum *>"]);
}

#[test]
fn test_type_298() {
    let result = parse_type("struct __cppobj hkArrayBase<hkgpConvexHull *>");
    assert_eq!(result, vec!["hkArrayBase<hkgpConvexHull *>"]);
}

#[test]
fn test_type_299() {
    let result =
        parse_type("struct __cppobj hkArrayBase<hkHandle<unsigned short,65535,hkndBodyUniqueId> >");
    assert_eq!(
        result,
        vec!["hkArrayBase<hkHandle<unsigned short,65535,hkndBodyUniqueId> >"]
    );
}

#[test]
fn test_type_300() {
    let result = parse_type("struct __cppobj hkArrayBase<hkMeshBody *>");
    assert_eq!(result, vec!["hkArrayBase<hkMeshBody *>"]);
}

#[test]
fn test_type_301() {
    let result = parse_type("struct __cppobj hkArrayBase<hkMonitorStreamStringMap::StringMap>");
    assert_eq!(
        result,
        vec!["hkArrayBase<hkMonitorStreamStringMap::StringMap>"]
    );
}

#[test]
fn test_type_302() {
    let result = parse_type("struct __cppobj hkArrayBase<hkndDecomposeImpl::ConnectionArea>");
    assert_eq!(
        result,
        vec!["hkArrayBase<hkndDecomposeImpl::ConnectionArea>"]
    );
}

#[test]
fn test_type_303() {
    let result = parse_type("struct __cppobj hkArrayBase<hkndNonManifoldConverter::EdgeOp>");
    assert_eq!(
        result,
        vec!["hkArrayBase<hkndNonManifoldConverter::EdgeOp>"]
    );
}

#[test]
fn test_type_304() {
    let result = parse_type("struct __cppobj hkArrayBase<hkpSerializedSubTrack1nInfo *>");
    assert_eq!(result, vec!["hkArrayBase<hkpSerializedSubTrack1nInfo *>"]);
}

#[test]
fn test_type_305() {
    let result = parse_type("struct __cppobj hkArrayBase<hkVertexSharingUtil::Entry>");
    assert_eq!(result, vec!["hkArrayBase<hkVertexSharingUtil::Entry>"]);
}

#[test]
fn test_type_306() {
    let result = parse_type(
        "struct __cppobj hkBlockStream<hknpShape::SdfContactPoint>::Writer : hkBlockStreamBase::Writer",
    );
    assert_eq!(
        result,
        vec!["hkBlockStream<hknpShape::SdfContactPoint>", "Writer"]
    );
}

#[test]
fn test_type_307() {
    let result = parse_type("struct __cppobj hkBlockStreamBase::Stream");
    assert_eq!(result, vec!["hkBlockStreamBase", "Stream"]);
}

#[test]
fn test_type_308() {
    let result = parse_type(
        "struct __cppobj hkcdSimdTreeUtils::Build::IRefit::GetClusterAabb<hkcdSimdTreeUtils::Build::GetLeafAabbFromAabbs>",
    );
    assert_eq!(
        result,
        vec![
            "hkcdSimdTreeUtils",
            "Build",
            "IRefit",
            "GetClusterAabb<hkcdSimdTreeUtils::Build::GetLeafAabbFromAabbs>"
        ]
    );
}

#[test]
fn test_type_309() {
    let result = parse_type(
        "struct __cppobj hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::BaseWrapper<hknpCompressedMeshShapeInternals::RayCastQuery<0> >",
    );
    assert_eq!(
        result,
        vec![
            "hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>",
            "BaseWrapper<hknpCompressedMeshShapeInternals::RayCastQuery<0> >"
        ]
    );
}

#[test]
fn test_type_310() {
    let result = parse_type(
        "struct __cppobj hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::HasGetLeafTree<hkcdStaticTree::DefaultTreeStorage6>",
    );
    assert_eq!(
        result,
        vec![
            "hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>",
            "HasGetLeafTree<hkcdStaticTree::DefaultTreeStorage6>"
        ]
    );
}

#[test]
fn test_type_311() {
    let result = parse_type(
        "struct __cppobj hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::IfBeginFront<hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::AabbOverlapsNoEarlyExitWrapper<hknpExternMeshShapeInternals::AabbOverlaps<hknpCollisionQueryCollector> > >",
    );
    assert_eq!(
        result,
        vec![
            "hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>",
            "IfBeginFront<hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::AabbOverlapsNoEarlyExitWrapper<hknpExternMeshShapeInternals::AabbOverlaps<hknpCollisionQueryCollector> > >"
        ]
    );
}

#[test]
fn test_type_312() {
    let result = parse_type(
        "struct __cppobj hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::IfEndFront<hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::AabbOverlapsNoEarlyExitWrapper<hknpCompoundShapeInternals<hknpDynamicCompoundShapeInternals>::AabbOverlaps<0,hkArray<unsigned int,hkContainerHeapAllocator> > > >",
    );
    assert_eq!(
        result,
        vec![
            "hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>",
            "IfEndFront<hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::AabbOverlapsNoEarlyExitWrapper<hknpCompoundShapeInternals<hknpDynamicCompoundShapeInternals>::AabbOverlaps<0,hkArray<unsigned int,hkContainerHeapAllocator> > > >"
        ]
    );
}

#[test]
fn test_type_313() {
    let result = parse_type(
        "struct __cppobj hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::IfEndFront<hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::AabbOverlapsWithEarlyExitWrapper<hkGeometryProcessingInternals::TJunctionsQuery> >",
    );
    assert_eq!(
        result,
        vec![
            "hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>",
            "IfEndFront<hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::AabbOverlapsWithEarlyExitWrapper<hkGeometryProcessingInternals::TJunctionsQuery> >"
        ]
    );
}

#[test]
fn test_type_314() {
    let result = parse_type(
        "struct __cppobj hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::IfFilterNode<hknpCompressedMeshShapeInternals::RayCastQuery<1>,hkcdStaticTree::Tree<hkcdStaticTree::DynamicStorage5>::NodeContext>",
    );
    assert_eq!(
        result,
        vec![
            "hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>",
            "IfFilterNode<hknpCompressedMeshShapeInternals::RayCastQuery<1>,hkcdStaticTree::Tree<hkcdStaticTree::DynamicStorage5>::NodeContext>"
        ]
    );
}

#[test]
fn test_type_315() {
    let result = parse_type(
        "struct __cppobj hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::IfGetFront<hknpExternMeshShapeInternals::GetClosestPointsToConvexQueryScaled>",
    );
    assert_eq!(
        result,
        vec![
            "hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>",
            "IfGetFront<hknpExternMeshShapeInternals::GetClosestPointsToConvexQueryScaled>"
        ]
    );
}

#[test]
fn test_type_316() {
    let result = parse_type(
        "struct __cppobj hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::IfPopNode<hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::ClosestFromAabbWrapper<hknpExternMeshShapeInternals::GetClosestPointsToConvexQueryUnscaled> >::Default",
    );
    assert_eq!(
        result,
        vec![
            "hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>",
            "IfPopNode<hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::ClosestFromAabbWrapper<hknpExternMeshShapeInternals::GetClosestPointsToConvexQueryUnscaled> >",
            "Default"
        ]
    );
}

#[test]
fn test_type_317() {
    let result = parse_type(
        "struct __cppobj hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::IfProcessChildren<hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::AabbOverlapsNearMissAabbWrapper<hknpCompressedMeshShapeInternals::AabbOverlaps<hknpCollisionQueryCollector> >,hknpCompressedMeshShapeTree>",
    );
    assert_eq!(
        result,
        vec![
            "hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>",
            "IfProcessChildren<hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::AabbOverlapsNearMissAabbWrapper<hknpCompressedMeshShapeInternals::AabbOverlaps<hknpCollisionQueryCollector> >,hknpCompressedMeshShapeTree>"
        ]
    );
}

#[test]
fn test_type_318() {
    let result = parse_type(
        "struct __cppobj hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::IfPushFront<hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::AabbOverlapsNearMissAabbWrapper<hknpCompressedMeshShapeInternals::AabbOverlaps<hknpCollisionQueryCollector> > >",
    );
    assert_eq!(
        result,
        vec![
            "hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>",
            "IfPushFront<hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::AabbOverlapsNearMissAabbWrapper<hknpCompressedMeshShapeInternals::AabbOverlaps<hknpCollisionQueryCollector> > >"
        ]
    );
}

#[test]
fn test_type_319() {
    let result = parse_type(
        "struct __cppobj hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::IfPushFront<hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::RayCastWrapperExternal<hknpCompoundShapeInternals<hknpStaticCompoundShapeInternals>::RayCast<0> > >",
    );
    assert_eq!(
        result,
        vec![
            "hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>",
            "IfPushFront<hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::RayCastWrapperExternal<hknpCompoundShapeInternals<hknpStaticCompoundShapeInternals>::RayCast<0> > >"
        ]
    );
}

#[test]
fn test_type_320() {
    let result = parse_type(
        "struct __cppobj hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::IfSelect<hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::AabbOverlapsNearMissAabbWrapper<hknpCompoundShapeInternals<hknpStaticCompoundShapeInternals>::AabbOverlaps<1,hkArray<unsigned int,hkContainerHeapAllocator> > > >::Default",
    );
    assert_eq!(
        result,
        vec![
            "hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>",
            "IfSelect<hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::AabbOverlapsNearMissAabbWrapper<hknpCompoundShapeInternals<hknpStaticCompoundShapeInternals>::AabbOverlaps<1,hkArray<unsigned int,hkContainerHeapAllocator> > > >",
            "Default"
        ]
    );
}

#[test]
fn test_type_321() {
    let result = parse_type(
        "struct __cppobj hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::IfSelect<hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::AabbOverlapsWithEarlyExitWrapper<hkgpConvexDecompositionImpl::OverlapGuard> >",
    );
    assert_eq!(
        result,
        vec![
            "hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>",
            "IfSelect<hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::AabbOverlapsWithEarlyExitWrapper<hkgpConvexDecompositionImpl::OverlapGuard> >"
        ]
    );
}

#[test]
fn test_type_322() {
    let result = parse_type(
        "struct __cppobj hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::SelectType<hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::IfEndFront<hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::AabbOverlapsWithEarlyExitWrapper<CollideShapeTriangle> >::Default,hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::IfEndFront<hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::AabbOverlapsWithEarlyExitWrapper<CollideShapeTriangle> >::Member,0>",
    );
    assert_eq!(
        result,
        vec![
            "hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>",
            "SelectType<hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::IfEndFront<hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::AabbOverlapsWithEarlyExitWrapper<CollideShapeTriangle> >::Default,hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::IfEndFront<hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::AabbOverlapsWithEarlyExitWrapper<CollideShapeTriangle> >::Member,0>"
        ]
    );
}

#[test]
fn test_type_323() {
    let result = parse_type(
        "struct __cppobj hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::SelectType<hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::IfPopNode<hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::AabbOverlapsNearMissAabbWrapper<hknpCompoundShapeInternals<hknpStaticCompoundShapeInternals>::AabbOverlaps<1,hkArray<unsigned int,hkContainerHeapAllocator> > > >::Default,hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::IfPopNode<hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::AabbOverlapsNearMissAabbWrapper<hknpCompoundShapeInternals<hknpStaticCompoundShapeInternals>::AabbOverlaps<1,hkArray<unsigned int,hkContainerHeapAllocator> > > >::Member,0>",
    );
    assert_eq!(
        result,
        vec![
            "hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>",
            "SelectType<hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::IfPopNode<hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::AabbOverlapsNearMissAabbWrapper<hknpCompoundShapeInternals<hknpStaticCompoundShapeInternals>::AabbOverlaps<1,hkArray<unsigned int,hkContainerHeapAllocator> > > >::Default,hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::IfPopNode<hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::AabbOverlapsNearMissAabbWrapper<hknpCompoundShapeInternals<hknpStaticCompoundShapeInternals>::AabbOverlaps<1,hkArray<unsigned int,hkContainerHeapAllocator> > > >::Member,0>"
        ]
    );
}

#[test]
fn test_type_324() {
    let result = parse_type(
        "struct __cppobj hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::SelectType<hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::IfPushFront<hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::AabbOverlapsNearMissAabbWrapper<hknpCompressedMeshShapeInternals::AabbOverlaps<hkArray<unsigned int,hkContainerHeapAllocator> > > >::Default,hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::IfPushFront<hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::AabbOverlapsNearMissAabbWrapper<hknpCompressedMeshShapeInternals::AabbOverlaps<hkArray<unsigned int,hkContainerHeapAllocator> > > >::Member,0>",
    );
    assert_eq!(
        result,
        vec![
            "hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>",
            "SelectType<hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::IfPushFront<hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::AabbOverlapsNearMissAabbWrapper<hknpCompressedMeshShapeInternals::AabbOverlaps<hkArray<unsigned int,hkContainerHeapAllocator> > > >::Default,hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::IfPushFront<hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::AabbOverlapsNearMissAabbWrapper<hknpCompressedMeshShapeInternals::AabbOverlaps<hkArray<unsigned int,hkContainerHeapAllocator> > > >::Member,0>"
        ]
    );
}

#[test]
fn test_type_325() {
    let result = parse_type(
        "struct __cppobj hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::SelectType<hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::IfPushFront<hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::AabbOverlapsNoEarlyExitWrapper<hknpExternMeshShapeInternals::AabbOverlaps<hkArray<unsigned int,hkContainerHeapAllocator> > > >::Default,hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::IfPushFront<hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::AabbOverlapsNoEarlyExitWrapper<hknpExternMeshShapeInternals::AabbOverlaps<hkArray<unsigned int,hkContainerHeapAllocator> > > >::Member,0>",
    );
    assert_eq!(
        result,
        vec![
            "hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>",
            "SelectType<hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::IfPushFront<hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::AabbOverlapsNoEarlyExitWrapper<hknpExternMeshShapeInternals::AabbOverlaps<hkArray<unsigned int,hkContainerHeapAllocator> > > >::Default,hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::IfPushFront<hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::AabbOverlapsNoEarlyExitWrapper<hknpExternMeshShapeInternals::AabbOverlaps<hkArray<unsigned int,hkContainerHeapAllocator> > > >::Member,0>"
        ]
    );
}

#[test]
fn test_type_326() {
    let result = parse_type(
        "struct __cppobj hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::SelectType<hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::IfSelect<hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::SphereCastWrapper<WrappedRaycastQuery<hkcdStaticTree::DefaultTreeStorage6> > >::Default,hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::IfSelect<hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::SphereCastWrapper<WrappedRaycastQuery<hkcdStaticTree::DefaultTreeStorage6> > >::Member,1>",
    );
    assert_eq!(
        result,
        vec![
            "hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>",
            "SelectType<hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::IfSelect<hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::SphereCastWrapper<WrappedRaycastQuery<hkcdStaticTree::DefaultTreeStorage6> > >::Default,hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::IfSelect<hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::SphereCastWrapper<WrappedRaycastQuery<hkcdStaticTree::DefaultTreeStorage6> > >::Member,1>"
        ]
    );
}

#[test]
fn test_type_327() {
    let result = parse_type(
        "struct __cppobj hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::unaryStackless<0,hknpCompressedMeshShapeTree,hknpCompressedMeshShapeInternals::GetClosestPointsToHeightFieldQueryScaled2>",
    );
    assert_eq!(
        result,
        vec![
            "hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>",
            "unaryStackless<0,hknpCompressedMeshShapeTree,hknpCompressedMeshShapeInternals::GetClosestPointsToHeightFieldQueryScaled2>"
        ]
    );
}

#[test]
fn test_type_328() {
    let result = parse_type(
        "struct __cppobj hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::UnaryTrampoline<hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>,1>::Forwarder<hknpCompressedMeshShapeTree,1>",
    );
    assert_eq!(
        result,
        vec![
            "hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>",
            "UnaryTrampoline<hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>,1>",
            "Forwarder<hknpCompressedMeshShapeTree,1>"
        ]
    );
}

#[test]
fn test_type_329() {
    let result = parse_type(
        "struct __cppobj hkcdTreeQueriesStacks::Dynamic<64,hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::SlotPair<hkcdDynamicTree::DefaultTree48Storage,hkcdDynamicTree::DefaultTree48Storage> >",
    );
    assert_eq!(
        result,
        vec![
            "hkcdTreeQueriesStacks",
            "Dynamic<64,hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::SlotPair<hkcdDynamicTree::DefaultTree48Storage,hkcdDynamicTree::DefaultTree48Storage> >"
        ]
    );
}

#[test]
fn test_type_330() {
    let result = parse_type("struct __cppobj hkCompileError::MX_SUBVECTORf_INDEX_OUT_OF_RANGE<1>");
    assert_eq!(
        result,
        vec!["hkCompileError", "MX_SUBVECTORf_INDEX_OUT_OF_RANGE<1>"]
    );
}

#[test]
fn test_type_331() {
    let result = parse_type("struct __cppobj hkdBreakableBody::ConstraintToChild");
    assert_eq!(result, vec!["hkdBreakableBody", "ConstraintToChild"]);
}

#[test]
fn test_type_332() {
    let result = parse_type(
        "struct __cppobj hkEnum<enum hkaPredictiveCompressedAnimation::StorageClass,int>",
    );
    assert_eq!(
        result,
        vec!["hkEnum<enum hkaPredictiveCompressedAnimation::StorageClass,int>"]
    );
}

#[test]
fn test_type_333() {
    let result = parse_type(
        "struct __cppobj hkEnum<enum hkgpTriangulatorType<hkContainerHeapAllocator,hkgpConvexDecompositionImpl::TriangulationVertex,hkgpTriangulatorBase::TriangleBase,hkgpTriangulatorBase::DefaultEdgeData<hkContainerHeapAllocator>,hkgpTriangulatorBase::SparseEdgeDataPolicy<hkgpTriangulatorBase::DefaultEdgeData<hkContainerHeapAllocator>,hkContainerHeapAllocator>,-1,4,15,0>::Insertion::eType,int>",
    );
    assert_eq!(
        result,
        vec![
            "hkEnum<enum hkgpTriangulatorType<hkContainerHeapAllocator,hkgpConvexDecompositionImpl::TriangulationVertex,hkgpTriangulatorBase::TriangleBase,hkgpTriangulatorBase::DefaultEdgeData<hkContainerHeapAllocator>,hkgpTriangulatorBase::SparseEdgeDataPolicy<hkgpTriangulatorBase::DefaultEdgeData<hkContainerHeapAllocator>,hkContainerHeapAllocator>,-1,4,15,0>::Insertion::eType,int>"
        ]
    );
}

#[test]
fn test_type_334() {
    let result = parse_type(
        "struct __cppobj hkEnum<enum hkgpTriangulatorType<hkContainerHeapAllocator,hkgpTriangulatorBase::VertexBase,hkgpTriangulatorBase::TriangleBase,hkgpTriangulatorBase::DefaultEdgeData<hkContainerHeapAllocator>,hkgpTriangulatorBase::NoEdgeDataPolicy<hkgpTriangulatorBase::DefaultEdgeData<hkContainerHeapAllocator>,hkContainerHeapAllocator>,-1,4,23,0>::Status,unsigned char>",
    );
    assert_eq!(
        result,
        vec![
            "hkEnum<enum hkgpTriangulatorType<hkContainerHeapAllocator,hkgpTriangulatorBase::VertexBase,hkgpTriangulatorBase::TriangleBase,hkgpTriangulatorBase::DefaultEdgeData<hkContainerHeapAllocator>,hkgpTriangulatorBase::NoEdgeDataPolicy<hkgpTriangulatorBase::DefaultEdgeData<hkContainerHeapAllocator>,hkContainerHeapAllocator>,-1,4,23,0>::Status,unsigned char>"
        ]
    );
}

#[test]
fn test_type_335() {
    let result = parse_type("struct __cppobj hkEnum<enum hknpWorld::SimulationStage,unsigned int>");
    assert_eq!(
        result,
        vec!["hkEnum<enum hknpWorld::SimulationStage,unsigned int>"]
    );
}

#[test]
fn test_type_336() {
    let result = parse_type("struct __cppobj hkEnum<enum hkpMaterial::ResponseType,signed char>");
    assert_eq!(
        result,
        vec!["hkEnum<enum hkpMaterial::ResponseType,signed char>"]
    );
}

#[test]
fn test_type_337() {
    let result = parse_type(
        "struct __cppobj hkEnum<enum hkpVelocityAccumulator::hkpAccumulatorType,unsigned char>",
    );
    assert_eq!(
        result,
        vec!["hkEnum<enum hkpVelocityAccumulator::hkpAccumulatorType,unsigned char>"]
    );
}

#[test]
fn test_type_338() {
    let result = parse_type("struct __cppobj hkFreeList");
    assert_eq!(result, vec!["hkFreeList"]);
}

#[test]
fn test_type_339() {
    let result = parse_type(
        "struct __cppobj hkGeometryProcessing::PoolAllocator<hkgpConvexHullImpl::Vertex,32,hkContainerHeapAllocator>",
    );
    assert_eq!(
        result,
        vec![
            "hkGeometryProcessing",
            "PoolAllocator<hkgpConvexHullImpl::Vertex,32,hkContainerHeapAllocator>"
        ]
    );
}

#[test]
fn test_type_340() {
    let result = parse_type(
        "struct __cppobj hkgpAbstractMesh<hkgpIndexedMeshDefinitions::Edge,hkgpIndexedMeshDefinitions::Vertex,hkgpIndexedMeshDefinitions::Triangle,hkContainerHeapAllocator>::TriangleIterator",
    );
    assert_eq!(
        result,
        vec![
            "hkgpAbstractMesh<hkgpIndexedMeshDefinitions::Edge,hkgpIndexedMeshDefinitions::Vertex,hkgpIndexedMeshDefinitions::Triangle,hkContainerHeapAllocator>",
            "TriangleIterator"
        ]
    );
}

#[test]
fn test_type_341() {
    let result = parse_type(
        "struct __cppobj hkgpAbstractMeshDefinitions::List<hkgpTriangulatorType<hkContainerHeapAllocator,hkgpTriangulatorBase::VertexBase,hkgpTriangulatorBase::TriangleBase,hkaiRuntimeTriangulatorEdgeData,hkgpTriangulatorBase::DenseEdgeDataPolicy<hkaiRuntimeTriangulatorEdgeData,hkContainerHeapAllocator>,-1,1,15,0>::Vertex,hkgpAbstractMeshDefinitions::List<hkgpTriangulatorType<hkContainerHeapAllocator,hkgpTriangulatorBase::VertexBase,hkgpTriangulatorBase::TriangleBase,hkaiRuntimeTriangulatorEdgeData,hkgpTriangulatorBase::DenseEdgeDataPolicy<hkaiRuntimeTriangulatorEdgeData,hkContainerHeapAllocator>,-1,1,15,0>::Vertex,hkgpTriangulatorBase::VertexBase,hkGeometryProcessing::PoolAllocator<hkgpTriangulatorType<hkContainerHeapAllocator,hkgpTriangulatorBase::VertexBase,hkgpTriangulatorBase::TriangleBase,hkaiRuntimeTriangulatorEdgeData,hkgpTriangulatorBase::DenseEdgeDataPolicy<hkaiRuntimeTriangulatorEdgeData,hkContainerHeapAllocator>,-1,1,15,0>::Vertex,32,hkContainerHeapAllocator> >::Item,hkGeometryProcessing::PoolAllocator<hkgpTriangulatorType<hkContainerHeapAllocator,hkgpTriangulatorBase::VertexBase,hkgpTriangulatorBase::TriangleBase,hkaiRuntimeTriangulatorEdgeData,hkgpTriangulatorBase::DenseEdgeDataPolicy<hkaiRuntimeTriangulatorEdgeData,hkContainerHeapAllocator>,-1,1,15,0>::Vertex,32,hkContainerHeapAllocator> >",
    );
    assert_eq!(
        result,
        vec![
            "hkgpAbstractMeshDefinitions",
            "List<hkgpTriangulatorType<hkContainerHeapAllocator,hkgpTriangulatorBase::VertexBase,hkgpTriangulatorBase::TriangleBase,hkaiRuntimeTriangulatorEdgeData,hkgpTriangulatorBase::DenseEdgeDataPolicy<hkaiRuntimeTriangulatorEdgeData,hkContainerHeapAllocator>,-1,1,15,0>::Vertex,hkgpAbstractMeshDefinitions::List<hkgpTriangulatorType<hkContainerHeapAllocator,hkgpTriangulatorBase::VertexBase,hkgpTriangulatorBase::TriangleBase,hkaiRuntimeTriangulatorEdgeData,hkgpTriangulatorBase::DenseEdgeDataPolicy<hkaiRuntimeTriangulatorEdgeData,hkContainerHeapAllocator>,-1,1,15,0>::Vertex,hkgpTriangulatorBase::VertexBase,hkGeometryProcessing::PoolAllocator<hkgpTriangulatorType<hkContainerHeapAllocator,hkgpTriangulatorBase::VertexBase,hkgpTriangulatorBase::TriangleBase,hkaiRuntimeTriangulatorEdgeData,hkgpTriangulatorBase::DenseEdgeDataPolicy<hkaiRuntimeTriangulatorEdgeData,hkContainerHeapAllocator>,-1,1,15,0>::Vertex,32,hkContainerHeapAllocator> >::Item,hkGeometryProcessing::PoolAllocator<hkgpTriangulatorType<hkContainerHeapAllocator,hkgpTriangulatorBase::VertexBase,hkgpTriangulatorBase::TriangleBase,hkaiRuntimeTriangulatorEdgeData,hkgpTriangulatorBase::DenseEdgeDataPolicy<hkaiRuntimeTriangulatorEdgeData,hkContainerHeapAllocator>,-1,1,15,0>::Vertex,32,hkContainerHeapAllocator> >"
        ]
    );
}

#[test]
fn test_type_342() {
    let result =
        parse_type("struct __cppobj hkgpConvexDecompositionImpl : hkgpConvexDecomposition");
    assert_eq!(result, vec!["hkgpConvexDecompositionImpl"]);
}

#[test]
fn test_type_343() {
    let result = parse_type("struct __cppobj hkgpMeshInternals::Ring::Segment");
    assert_eq!(result, vec!["hkgpMeshInternals", "Ring", "Segment"]);
}

#[test]
fn test_type_344() {
    let result = parse_type(
        "struct __cppobj hkgpTriangulatorType<hkContainerHeapAllocator,hkgpTriangulatorBase::VertexBase,hkgpTriangulatorBase::TriangleBase,hkgpTriangulatorBase::DefaultEdgeData<hkContainerHeapAllocator>,hkgpTriangulatorBase::SparseEdgeDataPolicy<hkgpTriangulatorBase::DefaultEdgeData<hkContainerHeapAllocator>,hkContainerHeapAllocator>,-1,4,29,0>::Location",
    );
    assert_eq!(
        result,
        vec![
            "hkgpTriangulatorType<hkContainerHeapAllocator,hkgpTriangulatorBase::VertexBase,hkgpTriangulatorBase::TriangleBase,hkgpTriangulatorBase::DefaultEdgeData<hkContainerHeapAllocator>,hkgpTriangulatorBase::SparseEdgeDataPolicy<hkgpTriangulatorBase::DefaultEdgeData<hkContainerHeapAllocator>,hkContainerHeapAllocator>,-1,4,29,0>",
            "Location"
        ]
    );
}

#[test]
fn test_type_345() {
    let result = parse_type(
        "struct __cppobj hkInplaceArray<hknpHybridAabbTree<unsigned int,unsigned int,hkAabb16>::Node const *,128,hkContainerTempAllocator> : hkArray<hknpHybridAabbTree<unsigned int,unsigned int,hkAabb16>::Node const *,hkContainerTempAllocator>",
    );
    assert_eq!(
        result,
        vec![
            "hkInplaceArray<hknpHybridAabbTree<unsigned int,unsigned int,hkAabb16>::Node const *,128,hkContainerTempAllocator>"
        ]
    );
}

#[test]
fn test_type_346() {
    let result = parse_type(
        "struct __cppobj hkInplaceArray<int,2,hkContainerHeapAllocator> : hkArray<int,hkContainerHeapAllocator>",
    );
    assert_eq!(
        result,
        vec!["hkInplaceArray<int,2,hkContainerHeapAllocator>"]
    );
}

#[test]
fn test_type_347() {
    let result = parse_type(
        "struct __cppobj hkMap<unsigned __int64,hkCheckingMemorySystem::AllocInfo,hkMapOperations<unsigned __int64>,hkContainerHeapAllocator> : hkMapBase<unsigned __int64,hkCheckingMemorySystem::AllocInfo,hkMapOperations<unsigned __int64> >",
    );
    assert_eq!(
        result,
        vec![
            "hkMap<unsigned __int64,hkCheckingMemorySystem::AllocInfo,hkMapOperations<unsigned __int64>,hkContainerHeapAllocator>"
        ]
    );
}

#[test]
fn test_type_348() {
    let result = parse_type(
        "struct __cppobj hkMapBase<hknpWorld *,perWorldInfo *,hkMapOperations<hknpWorld *> >",
    );
    assert_eq!(
        result,
        vec!["hkMapBase<hknpWorld *,perWorldInfo *,hkMapOperations<hknpWorld *> >"]
    );
}

#[test]
fn test_type_349() {
    let result = parse_type("struct __cppobj hkMonitorStreamFrameInfo");
    assert_eq!(result, vec!["hkMonitorStreamFrameInfo"]);
}

#[test]
fn test_type_350() {
    let result = parse_type("struct __cppobj hkMxRealf_Implementation::gatherWithOffsetHR<2,16>");
    assert_eq!(
        result,
        vec!["hkMxRealf_Implementation", "gatherWithOffsetHR<2,16>"]
    );
}

#[test]
fn test_type_351() {
    let result = parse_type("struct __cppobj hkMxVectorf_Implementation::lengthH<1,3,2,1>");
    assert_eq!(
        result,
        vec!["hkMxVectorf_Implementation", "lengthH<1,3,2,1>"]
    );
}

#[test]
fn test_type_352() {
    let result = parse_type("struct __cppobj hkMxVectorf_Implementation::scatterWithOffsetH<4,16>");
    assert_eq!(
        result,
        vec!["hkMxVectorf_Implementation", "scatterWithOffsetH<4,16>"]
    );
}

#[test]
fn test_type_353() {
    let result = parse_type(
        "struct __cppobj hkndBodyTimeoutRuntime::BreakableBodyTimeoutSignal : hkSignal2<hkndWorld *,hkHandle<unsigned short,65535,hkndBodyUniqueId> >",
    );
    assert_eq!(
        result,
        vec!["hkndBodyTimeoutRuntime", "BreakableBodyTimeoutSignal"]
    );
}

#[test]
fn test_type_354() {
    let result = parse_type(
        "struct __cppobj hkndDecalRuntimeImpl::CopyMapRasterFunctor : hkndDecalMapRuntime::MapRasterFunctor",
    );
    assert_eq!(result, vec!["hkndDecalRuntimeImpl", "CopyMapRasterFunctor"]);
}

#[test]
fn test_type_355() {
    let result = parse_type("struct __cppobj hkndImageVectorizer::VectorizationParameters");
    assert_eq!(
        result,
        vec!["hkndImageVectorizer", "VectorizationParameters"]
    );
}

#[test]
fn test_type_356() {
    let result = parse_type("struct __cppobj hkndSolidGeometryToShapeConverter");
    assert_eq!(result, vec!["hkndSolidGeometryToShapeConverter"]);
}

#[test]
fn test_type_357() {
    let result = parse_type("struct __cppobj hkndWorld::PostStepSignal : hkSignal1<hkndWorld *>");
    assert_eq!(result, vec!["hkndWorld", "PostStepSignal"]);
}

#[test]
fn test_type_358() {
    let result = parse_type("struct __cppobj hknpBodyManager::PropertyBuffer");
    assert_eq!(result, vec!["hknpBodyManager", "PropertyBuffer"]);
}

#[test]
fn test_type_359() {
    let result = parse_type("struct __cppobj hknpCachedVertexReader");
    assert_eq!(result, vec!["hknpCachedVertexReader"]);
}

#[test]
fn test_type_360() {
    let result = parse_type(
        "struct __cppobj hknpCompressedMeshShapeInternals::GetClosestPointsToMeshQueryScaled : hknpCompressedMeshShapeInternals::GetClosestPointsToMeshQueryBase",
    );
    assert_eq!(
        result,
        vec![
            "hknpCompressedMeshShapeInternals",
            "GetClosestPointsToMeshQueryScaled"
        ]
    );
}

#[test]
fn test_type_361() {
    let result =
        parse_type("struct __cppobj hknpDestructionBreakOffModifier::ContactEvent : hkCommand");
    assert_eq!(
        result,
        vec!["hknpDestructionBreakOffModifier", "ContactEvent"]
    );
}

#[test]
fn test_type_362() {
    let result = parse_type("struct __cppobj hknpDynamicCompoundShapeData : hkReferencedObject");
    assert_eq!(result, vec!["hknpDynamicCompoundShapeData"]);
}

#[test]
fn test_type_363() {
    let result = parse_type("struct __cppobj hknpRestitutionModifier : hknpModifier");
    assert_eq!(result, vec!["hknpRestitutionModifier"]);
}

#[test]
fn test_type_364() {
    let result = parse_type("struct __cppobj hkOstream::CustomFormater");
    assert_eq!(result, vec!["hkOstream", "CustomFormater"]);
}

#[test]
fn test_type_365() {
    let result = parse_type("struct __cppobj hkpBallSocketChainData::Runtime");
    assert_eq!(result, vec!["hkpBallSocketChainData", "Runtime"]);
}

#[test]
fn test_type_366() {
    let result = parse_type("struct __cppobj hkpCollisionFilterList : hkpCollisionFilter");
    assert_eq!(result, vec!["hkpCollisionFilterList"]);
}

#[test]
fn test_type_367() {
    let result = parse_type(
        "struct __cppobj hkpJacobian1dLinearLimitsSchema : hkpJacobianSchemaInfo::LinearLimits1D",
    );
    assert_eq!(result, vec!["hkpJacobian1dLinearLimitsSchema"]);
}

#[test]
fn test_type_368() {
    let result = parse_type("struct __cppobj hkpJacobianSchemaInfo::Friction2D");
    assert_eq!(result, vec!["hkpJacobianSchemaInfo", "Friction2D"]);
}

#[test]
fn test_type_369() {
    let result = parse_type("struct __cppobj hkPointerMultiMap<int,unsigned __int64>");
    assert_eq!(result, vec!["hkPointerMultiMap<int,unsigned __int64>"]);
}

#[test]
fn test_type_370() {
    let result = parse_type("struct __cppobj hkpWheelConstraintData : hkpConstraintData");
    assert_eq!(result, vec!["hkpWheelConstraintData"]);
}

#[test]
fn test_type_371() {
    let result = parse_type("struct __cppobj hkRefLoan<hkaiAvoidanceProperties const >");
    assert_eq!(result, vec!["hkRefLoan<hkaiAvoidanceProperties const >"]);
}

#[test]
fn test_type_372() {
    let result = parse_type("struct __cppobj hkRefLoan<hkaiStreamingCollection>");
    assert_eq!(result, vec!["hkRefLoan<hkaiStreamingCollection>"]);
}

#[test]
fn test_type_373() {
    let result = parse_type("struct __cppobj hkRefLoan<hkMeshBody>");
    assert_eq!(result, vec!["hkRefLoan<hkMeshBody>"]);
}

#[test]
fn test_type_374() {
    let result = parse_type("struct __cppobj hkRefLoan<hkndController>");
    assert_eq!(result, vec!["hkRefLoan<hkndController>"]);
}

#[test]
fn test_type_375() {
    let result = parse_type("struct __cppobj hkRefNew<hkaiNavMesh const >");
    assert_eq!(result, vec!["hkRefNew<hkaiNavMesh const >"]);
}

#[test]
fn test_type_376() {
    let result = parse_type("struct __cppobj hkRefNew<hkaiVolumePathfindingUtil::FindPathOutput>");
    assert_eq!(
        result,
        vec!["hkRefNew<hkaiVolumePathfindingUtil::FindPathOutput>"]
    );
}

#[test]
fn test_type_377() {
    let result = parse_type("struct __cppobj hkRefNew<hkndFractureLineMapImpl::MeshSection>");
    assert_eq!(
        result,
        vec!["hkRefNew<hkndFractureLineMapImpl::MeshSection>"]
    );
}

#[test]
fn test_type_378() {
    let result = parse_type("struct __cppobj hkRefPtr<hkaAnimationBinding>");
    assert_eq!(result, vec!["hkRefPtr<hkaAnimationBinding>"]);
}

#[test]
fn test_type_379() {
    let result = parse_type("struct __cppobj hkRefPtr<hkaiSilhouettePriorityController const >");
    assert_eq!(
        result,
        vec!["hkRefPtr<hkaiSilhouettePriorityController const >"]
    );
}

#[test]
fn test_type_380() {
    let result = parse_type("struct __cppobj hkRefPtr<hkndHierarchyInstance const >");
    assert_eq!(result, vec!["hkRefPtr<hkndHierarchyInstance const >"]);
}

#[test]
fn test_type_381() {
    let result = parse_type("struct __cppobj hkRefPtr<hknpCharacterRigidBodyListener>");
    assert_eq!(result, vec!["hkRefPtr<hknpCharacterRigidBodyListener>"]);
}

#[test]
fn test_type_382() {
    let result = parse_type("struct __cppobj hkRefPtr<hkpBreakableBody::Controller>");
    assert_eq!(result, vec!["hkRefPtr<hkpBreakableBody::Controller>"]);
}

#[test]
fn test_type_383() {
    let result = parse_type("struct __cppobj hkRefPtr<hkpBreakableMultiMaterial::InverseMapping>");
    assert_eq!(
        result,
        vec!["hkRefPtr<hkpBreakableMultiMaterial::InverseMapping>"]
    );
}

#[test]
fn test_type_384() {
    let result = parse_type(
        "struct __cppobj hkSignal1<hkndWorld *>::MemberSlot<hkndIntegrityAnalyzerRuntime,void (__cdecl hkndIntegrityAnalyzerRuntime::*)(hkndWorld *)> : hkSignal1<hkndWorld *>::Slot",
    );
    assert_eq!(
        result,
        vec![
            "hkSignal1<hkndWorld *>",
            "MemberSlot<hkndIntegrityAnalyzerRuntime,void (__cdecl hkndIntegrityAnalyzerRuntime::*)(hkndWorld *)>"
        ]
    );
}

#[test]
fn test_type_385() {
    let result = parse_type("struct __cppobj hkSimdReal_AdvancedInterface::unrollf_loadH<0>");
    assert_eq!(
        result,
        vec!["hkSimdReal_AdvancedInterface", "unrollf_loadH<0>"]
    );
}

#[test]
fn test_type_386() {
    let result = parse_type("struct __cppobj hkSimdReal_AdvancedInterface::unrollf_setDiv<1,1>");
    assert_eq!(
        result,
        vec!["hkSimdReal_AdvancedInterface", "unrollf_setDiv<1,1>"]
    );
}

#[test]
fn test_type_387() {
    let result = parse_type("struct __cppobj hkSymmetricMatrixUtilImpl<float>");
    assert_eq!(result, vec!["hkSymmetricMatrixUtilImpl<float>"]);
}

#[test]
fn test_type_388() {
    let result = parse_type("struct __cppobj hkTrait::RemoveConst<hknpMotionId>");
    assert_eq!(result, vec!["hkTrait", "RemoveConst<hknpMotionId>"]);
}

#[test]
fn test_type_389() {
    let result = parse_type("struct __cppobj hkVector4_AdvancedInterface::unrolld_loadF16<4,0>");
    assert_eq!(
        result,
        vec!["hkVector4_AdvancedInterface", "unrolld_loadF16<4,0>"]
    );
}

#[test]
fn test_type_390() {
    let result = parse_type(
        "struct __cppobj hkWorldOperation::ActivateEntity : hkWorldOperation::BaseOperation",
    );
    assert_eq!(result, vec!["hkWorldOperation", "ActivateEntity"]);
}

#[test]
fn test_type_391() {
    let result = parse_type(
        "struct __cppobj hkWorldOperation::MergeIslands : hkWorldOperation::BaseOperation",
    );
    assert_eq!(result, vec!["hkWorldOperation", "MergeIslands"]);
}

#[test]
fn test_type_392() {
    let result = parse_type("struct __cppobj ID3D11DomainShader : ID3D11DeviceChild");
    assert_eq!(result, vec!["ID3D11DomainShader"]);
}

#[test]
fn test_type_393() {
    let result = parse_type("struct __cppobj index_ptr<NAnimationSystem::CSubtractRotationsBox>");
    assert_eq!(
        result,
        vec!["index_ptr<NAnimationSystem::CSubtractRotationsBox>"]
    );
}

#[test]
fn test_type_394() {
    let result = parse_type("struct __cppobj INetworkConnection : IDispatch");
    assert_eq!(result, vec!["INetworkConnection"]);
}

#[test]
fn test_type_395() {
    let result = parse_type("struct __cppobj Input::CInputDevice : Input::IInputDevice");
    assert_eq!(result, vec!["Input", "CInputDevice"]);
}

#[test]
fn test_type_396() {
    let result = parse_type("struct __cppobj int3");
    assert_eq!(result, vec!["int3"]);
}

#[test]
fn test_type_397() {
    let result = parse_type("struct __cppobj IStatisticProvider::SEntry");
    assert_eq!(result, vec!["IStatisticProvider", "SEntry"]);
}

#[test]
fn test_type_398() {
    let result = parse_type(
        "struct __cppobj NAnimationSystem::CBinaryBoxBase<NAnimationSystem::CNorBox,float,float,float> : NAnimationSystem::CBoxBase",
    );
    assert_eq!(
        result,
        vec![
            "NAnimationSystem",
            "CBinaryBoxBase<NAnimationSystem::CNorBox,float,float,float>"
        ]
    );
}

#[test]
fn test_type_399() {
    let result = parse_type(
        "struct __cppobj NAnimationSystem::CBinaryBoxBase<NAnimationSystem::COrBox,float,float,float> : NAnimationSystem::CBoxBase",
    );
    assert_eq!(
        result,
        vec![
            "NAnimationSystem",
            "CBinaryBoxBase<NAnimationSystem::COrBox,float,float,float>"
        ]
    );
}

#[test]
fn test_type_400() {
    let result = parse_type("struct __cppobj NAr::detail::<lambda118>");
    assert_eq!(result, vec!["NAr", "detail", "<lambda118>"]);
}

#[test]
fn test_type_401() {
    let result =
        parse_type("struct __cppobj NAr::detail::construct<std::pair<CStatisticContext,float> >");
    assert_eq!(
        result,
        vec![
            "NAr",
            "detail",
            "construct<std::pair<CStatisticContext,float> >"
        ]
    );
}

#[test]
fn test_type_402() {
    let result = parse_type("struct __cppobj NGameSoundLibrary::SCirclePreCalc");
    assert_eq!(result, vec!["NGameSoundLibrary", "SCirclePreCalc"]);
}

#[test]
fn test_type_403() {
    let result = parse_type("struct __cppobj NGraphicsEngine::<lambda37>");
    assert_eq!(result, vec!["NGraphicsEngine", "<lambda37>"]);
}

#[test]
fn test_type_404() {
    let result = parse_type(
        "struct __cppobj NGraphicsEngine::COccluderCollectionManager : Base::CSingle<NGraphicsEngine::COccluderCollectionManager>",
    );
    assert_eq!(
        result,
        vec!["NGraphicsEngine", "COccluderCollectionManager"]
    );
}

#[test]
fn test_type_405() {
    let result = parse_type("struct __cppobj NGraphicsEngine::CompareTypeThenSortId");
    assert_eq!(result, vec!["NGraphicsEngine", "CompareTypeThenSortId"]);
}

#[test]
fn test_type_406() {
    let result = parse_type(
        "struct __cppobj NGraphicsEngine::CRenderBlockBavariumShield::CRenderBlockTypeBavariumShield : NGraphicsEngine::CRenderBlockType",
    );
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
fn test_type_407() {
    let result = parse_type("struct __cppobj NGraphicsEngine::CRenderBlockFont::STextBuffer");
    assert_eq!(
        result,
        vec!["NGraphicsEngine", "CRenderBlockFont", "STextBuffer"]
    );
}

#[test]
fn test_type_408() {
    let result = parse_type(
        "struct __cppobj NGraphicsEngine::CRenderBlockLRCloudsCompose : NGraphicsEngine::CRenderBlock",
    );
    assert_eq!(
        result,
        vec!["NGraphicsEngine", "CRenderBlockLRCloudsCompose"]
    );
}

#[test]
fn test_type_409() {
    let result = parse_type("struct __cppobj NGraphScript::CProcessor");
    assert_eq!(result, vec!["NGraphScript", "CProcessor"]);
}

#[test]
fn test_type_410() {
    let result = parse_type("struct __cppobj NGSONodes::CNodeCounterHandler");
    assert_eq!(result, vec!["NGSONodes", "CNodeCounterHandler"]);
}

#[test]
fn test_type_411() {
    let result = parse_type("struct __cppobj NInputTasks::NRelease::SProperties");
    assert_eq!(result, vec!["NInputTasks", "NRelease", "SProperties"]);
}

#[test]
fn test_type_412() {
    let result = parse_type("struct __cppobj NOnline::<lambda245>");
    assert_eq!(result, vec!["NOnline", "<lambda245>"]);
}

#[test]
fn test_type_413() {
    let result = parse_type("struct __cppobj NOnline::<lambda45>");
    assert_eq!(result, vec!["NOnline", "<lambda45>"]);
}

#[test]
fn test_type_414() {
    let result = parse_type(
        "struct __cppobj NSaveLoad::CManagedLocalObjectRef<CPlayerActionObserver::CSystemicButtonHintData> : NSaveLoad::CManagedObject<NSaveLoad::CLocalDataRef<CPlayerActionObserver::CSystemicButtonHintData> >",
    );
    assert_eq!(
        result,
        vec![
            "NSaveLoad",
            "CManagedLocalObjectRef<CPlayerActionObserver::CSystemicButtonHintData>"
        ]
    );
}

#[test]
fn test_type_415() {
    let result = parse_type("struct __cppobj ObjectConverter::ReplacedGeometry");
    assert_eq!(result, vec!["ObjectConverter", "ReplacedGeometry"]);
}

#[test]
fn test_type_416() {
    let result = parse_type(
        "struct __cppobj OSuite::TConstIterator<OSuite::TList<OSuite::ZOEdmFunctionImport *>::ZIterator,OSuite::ZOEdmFunctionImport *,int> : OSuite::ZObject",
    );
    assert_eq!(
        result,
        vec![
            "OSuite",
            "TConstIterator<OSuite::TList<OSuite::ZOEdmFunctionImport *>::ZIterator,OSuite::ZOEdmFunctionImport *,int>"
        ]
    );
}

#[test]
fn test_type_417() {
    let result = parse_type(
        "struct __cppobj OSuite::TList<OSuite::ManualRefCount<OSuite::IHttpRequest> >::ZIterator : OSuite::ZListBase::ZListIteratorBase",
    );
    assert_eq!(
        result,
        vec![
            "OSuite",
            "TList<OSuite::ManualRefCount<OSuite::IHttpRequest> >",
            "ZIterator"
        ]
    );
}

#[test]
fn test_type_418() {
    let result = parse_type(
        "struct __cppobj OSuite::TList<OSuitePrivate::OSTransaction *>::ZIterator : OSuite::ZListBase::ZListIteratorBase",
    );
    assert_eq!(
        result,
        vec![
            "OSuite",
            "TList<OSuitePrivate::OSTransaction *>",
            "ZIterator"
        ]
    );
}

#[test]
fn test_type_419() {
    let result = parse_type(
        "struct __cppobj OSuite::TPair<enum OSuite::ZError::EError,OSuite::ZString> : OSuite::ZObject",
    );
    assert_eq!(
        result,
        vec![
            "OSuite",
            "TPair<enum OSuite::ZError::EError,OSuite::ZString>"
        ]
    );
}

#[test]
fn test_type_420() {
    let result = parse_type("struct __cppobj OSuite::ZError : OSuite::ZObject");
    assert_eq!(result, vec!["OSuite", "ZError"]);
}

#[test]
fn test_type_421() {
    let result = parse_type(
        "struct __cppobj rapidjson::GenericMemberIterator<0,rapidjson::UTF8<char>,rapidjson::MemoryPoolAllocator<rapidjson::JsonAllocatorWrapper> > : std::iterator<std::random_access_iterator_tag,rapidjson::GenericMember<rapidjson::UTF8<char>,rapidjson::MemoryPoolAllocator<rapidjson::JsonAllocatorWrapper> >,__int64,rapidjson::GenericMember<rapidjson::UTF8<char>,rapidjson::MemoryPoolAllocator<rapidjson::JsonAllocatorWrapper> > *,rapidjson::GenericMember<rapidjson::UTF8<char>,rapidjson::MemoryPoolAllocator<rapidjson::JsonAllocatorWrapper> > &>",
    );
    assert_eq!(
        result,
        vec![
            "rapidjson",
            "GenericMemberIterator<0,rapidjson::UTF8<char>,rapidjson::MemoryPoolAllocator<rapidjson::JsonAllocatorWrapper> >"
        ]
    );
}

#[test]
fn test_type_422() {
    let result = parse_type(
        "struct __cppobj Scaleform::AllocatorDH<Scaleform::GFx::AS3::Value const *,2> : Scaleform::AllocatorBaseDH<2>, Scaleform::ConstructorMov<Scaleform::GFx::AS3::Value const *>",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "AllocatorDH<Scaleform::GFx::AS3::Value const *,2>"
        ]
    );
}

#[test]
fn test_type_423() {
    let result = parse_type(
        "struct __cppobj Scaleform::AllocatorLH<Scaleform::Ptr<Scaleform::GFx::LoaderImpl::HttpFileCache>,2> : Scaleform::AllocatorBaseLH<2>, Scaleform::ConstructorMov<Scaleform::Ptr<Scaleform::GFx::LoaderImpl::HttpFileCache> >",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "AllocatorLH<Scaleform::Ptr<Scaleform::GFx::LoaderImpl::HttpFileCache>,2>"
        ]
    );
}

#[test]
fn test_type_424() {
    let result = parse_type(
        "struct __cppobj Scaleform::ArrayBase<Scaleform::ArrayData<Scaleform::GFx::ImportData::Symbol,Scaleform::AllocatorLH<Scaleform::GFx::ImportData::Symbol,2>,Scaleform::ArrayDefaultPolicy> >",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "ArrayBase<Scaleform::ArrayData<Scaleform::GFx::ImportData::Symbol,Scaleform::AllocatorLH<Scaleform::GFx::ImportData::Symbol,2>,Scaleform::ArrayDefaultPolicy> >"
        ]
    );
}

#[test]
fn test_type_425() {
    let result = parse_type(
        "struct __cppobj Scaleform::ArrayBase<Scaleform::ArrayData<Scaleform::Ptr<Scaleform::GFx::ASIntervalTimerIntf>,Scaleform::AllocatorLH<Scaleform::Ptr<Scaleform::GFx::ASIntervalTimerIntf>,327>,Scaleform::ArrayDefaultPolicy> >",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "ArrayBase<Scaleform::ArrayData<Scaleform::Ptr<Scaleform::GFx::ASIntervalTimerIntf>,Scaleform::AllocatorLH<Scaleform::Ptr<Scaleform::GFx::ASIntervalTimerIntf>,327>,Scaleform::ArrayDefaultPolicy> >"
        ]
    );
}

#[test]
fn test_type_426() {
    let result = parse_type(
        "struct __cppobj Scaleform::ArrayBase<Scaleform::ArrayData<unsigned char,Scaleform::AllocatorGH_POD<unsigned char,2>,Scaleform::ArrayDefaultPolicy> >",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "ArrayBase<Scaleform::ArrayData<unsigned char,Scaleform::AllocatorGH_POD<unsigned char,2>,Scaleform::ArrayDefaultPolicy> >"
        ]
    );
}

#[test]
fn test_type_427() {
    let result = parse_type(
        "struct __cppobj Scaleform::ArrayBase<Scaleform::ArrayDataDHCC<Scaleform::GFx::ASString,Scaleform::AllocatorDH<Scaleform::GFx::ASString,2>,Scaleform::ArrayDefaultPolicy> >",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "ArrayBase<Scaleform::ArrayDataDHCC<Scaleform::GFx::ASString,Scaleform::AllocatorDH<Scaleform::GFx::ASString,2>,Scaleform::ArrayDefaultPolicy> >"
        ]
    );
}

#[test]
fn test_type_428() {
    let result = parse_type(
        "struct __cppobj Scaleform::ArrayData<Scaleform::Render::TextureFormat *,Scaleform::AllocatorLH<Scaleform::Render::TextureFormat *,2>,Scaleform::ArrayDefaultPolicy> : Scaleform::ArrayDataBase<Scaleform::Render::TextureFormat *,Scaleform::AllocatorLH<Scaleform::Render::TextureFormat *,2>,Scaleform::ArrayDefaultPolicy>",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "ArrayData<Scaleform::Render::TextureFormat *,Scaleform::AllocatorLH<Scaleform::Render::TextureFormat *,2>,Scaleform::ArrayDefaultPolicy>"
        ]
    );
}

#[test]
fn test_type_429() {
    let result = parse_type(
        "struct __cppobj Scaleform::ArrayDataBase<Scaleform::GFx::MovieDataDef::FrameLabelInfo,Scaleform::AllocatorDH<Scaleform::GFx::MovieDataDef::FrameLabelInfo,2>,Scaleform::ArrayDefaultPolicy>",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "ArrayDataBase<Scaleform::GFx::MovieDataDef::FrameLabelInfo,Scaleform::AllocatorDH<Scaleform::GFx::MovieDataDef::FrameLabelInfo,2>,Scaleform::ArrayDefaultPolicy>"
        ]
    );
}

#[test]
fn test_type_430() {
    let result = parse_type(
        "struct __cppobj Scaleform::ArrayDataBase<Scaleform::Ptr<Scaleform::GFx::ShapeDataBase>,Scaleform::AllocatorLH<Scaleform::Ptr<Scaleform::GFx::ShapeDataBase>,261>,Scaleform::ArrayDefaultPolicy>",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "ArrayDataBase<Scaleform::Ptr<Scaleform::GFx::ShapeDataBase>,Scaleform::AllocatorLH<Scaleform::Ptr<Scaleform::GFx::ShapeDataBase>,261>,Scaleform::ArrayDefaultPolicy>"
        ]
    );
}

#[test]
fn test_type_431() {
    let result = parse_type(
        "struct __cppobj Scaleform::ArrayDH_POD<enum Scaleform::GFx::AS3::XMLParser::Kind,2,Scaleform::ArrayDefaultPolicy> : Scaleform::ArrayBase<Scaleform::ArrayDataDH<enum Scaleform::GFx::AS3::XMLParser::Kind,Scaleform::AllocatorDH_POD<enum Scaleform::GFx::AS3::XMLParser::Kind,2>,Scaleform::ArrayDefaultPolicy> >",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "ArrayDH_POD<enum Scaleform::GFx::AS3::XMLParser::Kind,2,Scaleform::ArrayDefaultPolicy>"
        ]
    );
}

#[test]
fn test_type_432() {
    let result = parse_type(
        "struct __cppobj Scaleform::ArrayLH_CC<Scaleform::GFx::ASString,323,Scaleform::ArrayDefaultPolicy> : Scaleform::ArrayBase<Scaleform::ArrayDataLHCC<Scaleform::GFx::ASString,Scaleform::AllocatorLH<Scaleform::GFx::ASString,323>,Scaleform::ArrayDefaultPolicy> >",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "ArrayLH_CC<Scaleform::GFx::ASString,323,Scaleform::ArrayDefaultPolicy>"
        ]
    );
}

#[test]
fn test_type_433() {
    let result = parse_type(
        "struct __cppobj Scaleform::ArrayLH_POD<unsigned int,2,Scaleform::ArrayDefaultPolicy> : Scaleform::ArrayBase<Scaleform::ArrayData<unsigned int,Scaleform::AllocatorLH_POD<unsigned int,2>,Scaleform::ArrayDefaultPolicy> >",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "ArrayLH_POD<unsigned int,2,Scaleform::ArrayDefaultPolicy>"
        ]
    );
}

#[test]
fn test_type_434() {
    let result = parse_type("struct __cppobj Scaleform::ArrayStaticBuffPOD<unsigned short,72,2>");
    assert_eq!(
        result,
        vec!["Scaleform", "ArrayStaticBuffPOD<unsigned short,72,2>"]
    );
}

#[test]
fn test_type_435() {
    let result = parse_type(
        "struct __cppobj Scaleform::ConstructorMov<Scaleform::GFx::Text::CSSToken<wchar_t> >",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "ConstructorMov<Scaleform::GFx::Text::CSSToken<wchar_t> >"
        ]
    );
}

#[test]
fn test_type_436() {
    let result = parse_type(
        "struct __cppobj Scaleform::ConstructorMov<Scaleform::Ptr<Scaleform::GFx::ButtonActionBase> >",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "ConstructorMov<Scaleform::Ptr<Scaleform::GFx::ButtonActionBase> >"
        ]
    );
}

#[test]
fn test_type_437() {
    let result = parse_type(
        "struct __cppobj Scaleform::ConstructorMov<Scaleform::Ptr<Scaleform::GFx::DisplayObjectBase> >",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "ConstructorMov<Scaleform::Ptr<Scaleform::GFx::DisplayObjectBase> >"
        ]
    );
}

#[test]
fn test_type_438() {
    let result = parse_type(
        "struct __cppobj Scaleform::GFx::AMP::MessageProfileFrame : Scaleform::GFx::AMP::Message",
    );
    assert_eq!(
        result,
        vec!["Scaleform", "GFx", "AMP", "MessageProfileFrame"]
    );
}

#[test]
fn test_type_439() {
    let result = parse_type(
        "struct __cppobj Scaleform::GFx::AS3::Classes::fl_desktop::Clipboard : Scaleform::GFx::AS3::Class",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "GFx",
            "AS3",
            "Classes",
            "fl_desktop",
            "Clipboard"
        ]
    );
}

#[test]
fn test_type_440() {
    let result = parse_type(
        "struct __cppobj Scaleform::GFx::AS3::Classes::fl_text::Font::enumerateFonts::__l2::FontsVisitor : Scaleform::GFx::MovieDef::ResourceVisitor",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "GFx",
            "AS3",
            "Classes",
            "fl_text",
            "Font",
            "enumerateFonts",
            "__l2",
            "FontsVisitor"
        ]
    );
}

#[test]
fn test_type_441() {
    let result = parse_type(
        "struct __cppobj Scaleform::GFx::AS3::Classes::fl_text::FontStyle : Scaleform::GFx::AS3::Class",
    );
    assert_eq!(
        result,
        vec!["Scaleform", "GFx", "AS3", "Classes", "fl_text", "FontStyle"]
    );
}

#[test]
fn test_type_442() {
    let result = parse_type(
        "struct __cppobj Scaleform::GFx::AS3::ClassTraits::fl_desktop::DockIcon : Scaleform::GFx::AS3::ClassTraits::fl_desktop::InteractiveIcon",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "GFx",
            "AS3",
            "ClassTraits",
            "fl_desktop",
            "DockIcon"
        ]
    );
}

#[test]
fn test_type_443() {
    let result = parse_type(
        "struct __cppobj Scaleform::GFx::AS3::ClassTraits::fl_events::MouseEvent : Scaleform::GFx::AS3::ClassTraits::fl_events::Event",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "GFx",
            "AS3",
            "ClassTraits",
            "fl_events",
            "MouseEvent"
        ]
    );
}

#[test]
fn test_type_444() {
    let result = parse_type(
        "struct __cppobj Scaleform::GFx::AS3::ClassTraits::fl_gfx::DisplayObjectEx : Scaleform::GFx::AS3::ClassTraits::fl::Object",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "GFx",
            "AS3",
            "ClassTraits",
            "fl_gfx",
            "DisplayObjectEx"
        ]
    );
}

#[test]
fn test_type_445() {
    let result = parse_type(
        "struct __cppobj Scaleform::GFx::AS3::ClassTraits::fl_media::Camera : Scaleform::GFx::AS3::ClassTraits::fl_events::EventDispatcher",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "GFx",
            "AS3",
            "ClassTraits",
            "fl_media",
            "Camera"
        ]
    );
}

#[test]
fn test_type_446() {
    let result = parse_type(
        "struct __cppobj Scaleform::GFx::AS3::Impl::Coerce<Scaleform::GFx::AS3::Value,Scaleform::GFx::ASString>",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "GFx",
            "AS3",
            "Impl",
            "Coerce<Scaleform::GFx::AS3::Value,Scaleform::GFx::ASString>"
        ]
    );
}

#[test]
fn test_type_447() {
    let result = parse_type(
        "struct __cppobj Scaleform::GFx::AS3::Impl::ConvertUnsafe<Scaleform::GFx::AS3::Value,Scaleform::GFx::AS3::Value>",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "GFx",
            "AS3",
            "Impl",
            "ConvertUnsafe<Scaleform::GFx::AS3::Value,Scaleform::GFx::AS3::Value>"
        ]
    );
}

#[test]
fn test_type_448() {
    let result = parse_type(
        "struct __cppobj Scaleform::GFx::AS3::Instances::fl_net::URLRequest : Scaleform::GFx::AS3::Instances::fl::Object",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "GFx",
            "AS3",
            "Instances",
            "fl_net",
            "URLRequest"
        ]
    );
}

#[test]
fn test_type_449() {
    let result = parse_type(
        "struct __cppobj Scaleform::GFx::AS3::InstanceTraits::fl_sampler::StackFrame : Scaleform::GFx::AS3::InstanceTraits::fl::Object",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "GFx",
            "AS3",
            "InstanceTraits",
            "fl_sampler",
            "StackFrame"
        ]
    );
}

#[test]
fn test_type_450() {
    let result = parse_type(
        "struct __cppobj Scaleform::GFx::AS3::InstanceTraits::fl_xml::XMLDocument : Scaleform::GFx::AS3::InstanceTraits::fl_xml::XMLNode",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "GFx",
            "AS3",
            "InstanceTraits",
            "fl_xml",
            "XMLDocument"
        ]
    );
}

#[test]
fn test_type_451() {
    let result = parse_type(
        "struct __cppobj Scaleform::GFx::AS3::InstanceTraits::fl::Function : Scaleform::GFx::AS3::InstanceTraits::fl::Object",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "GFx",
            "AS3",
            "InstanceTraits",
            "fl",
            "Function"
        ]
    );
}

#[test]
fn test_type_452() {
    let result = parse_type(
        "struct __cppobj Scaleform::GFx::AS3::SPtr<Scaleform::GFx::AS3::ClassTraits::ClassClass>",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "GFx",
            "AS3",
            "SPtr<Scaleform::GFx::AS3::ClassTraits::ClassClass>"
        ]
    );
}

#[test]
fn test_type_453() {
    let result = parse_type(
        "struct __cppobj Scaleform::GFx::AS3::SPtr<Scaleform::GFx::AS3::Instances::fl_display::DisplayObjectContainer>",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "GFx",
            "AS3",
            "SPtr<Scaleform::GFx::AS3::Instances::fl_display::DisplayObjectContainer>"
        ]
    );
}

#[test]
fn test_type_454() {
    let result = parse_type(
        "struct __cppobj Scaleform::GFx::AS3::SPtr<Scaleform::GFx::AS3::Instances::fl_filters::BlurFilter>",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "GFx",
            "AS3",
            "SPtr<Scaleform::GFx::AS3::Instances::fl_filters::BlurFilter>"
        ]
    );
}

#[test]
fn test_type_455() {
    let result = parse_type(
        "struct __cppobj Scaleform::GFx::AS3::ThunkFunc0<Scaleform::GFx::AS3::Classes::fl_filesystem::File,10,Scaleform::GFx::AS3::SPtr<Scaleform::GFx::AS3::Instances::fl_filesystem::File> >",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "GFx",
            "AS3",
            "ThunkFunc0<Scaleform::GFx::AS3::Classes::fl_filesystem::File,10,Scaleform::GFx::AS3::SPtr<Scaleform::GFx::AS3::Instances::fl_filesystem::File> >"
        ]
    );
}

#[test]
fn test_type_456() {
    let result = parse_type(
        "struct __cppobj Scaleform::GFx::AS3::ThunkFunc0<Scaleform::GFx::AS3::Instances::fl_display::MovieClip,18,Scaleform::GFx::AS3::Value const >",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "GFx",
            "AS3",
            "ThunkFunc0<Scaleform::GFx::AS3::Instances::fl_display::MovieClip,18,Scaleform::GFx::AS3::Value const >"
        ]
    );
}

#[test]
fn test_type_457() {
    let result = parse_type(
        "struct __cppobj Scaleform::GFx::AS3::ThunkFunc0<Scaleform::GFx::AS3::Instances::fl_display::Stage,15,unsigned long>",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "GFx",
            "AS3",
            "ThunkFunc0<Scaleform::GFx::AS3::Instances::fl_display::Stage,15,unsigned long>"
        ]
    );
}

#[test]
fn test_type_458() {
    let result = parse_type(
        "struct __cppobj Scaleform::GFx::AS3::ThunkFunc0<Scaleform::GFx::AS3::Instances::fl_events::TouchEvent,12,double>",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "GFx",
            "AS3",
            "ThunkFunc0<Scaleform::GFx::AS3::Instances::fl_events::TouchEvent,12,double>"
        ]
    );
}

#[test]
fn test_type_459() {
    let result = parse_type(
        "struct __cppobj Scaleform::GFx::AS3::ThunkFunc0<Scaleform::GFx::AS3::Instances::fl_events::TransformGestureEvent,8,double>",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "GFx",
            "AS3",
            "ThunkFunc0<Scaleform::GFx::AS3::Instances::fl_events::TransformGestureEvent,8,double>"
        ]
    );
}

#[test]
fn test_type_460() {
    let result = parse_type(
        "struct __cppobj Scaleform::GFx::AS3::ThunkFunc0<Scaleform::GFx::AS3::Instances::fl_filters::GradientGlowFilter,10,double>",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "GFx",
            "AS3",
            "ThunkFunc0<Scaleform::GFx::AS3::Instances::fl_filters::GradientGlowFilter,10,double>"
        ]
    );
}

#[test]
fn test_type_461() {
    let result = parse_type(
        "struct __cppobj Scaleform::GFx::AS3::ThunkFunc0<Scaleform::GFx::AS3::Instances::fl_geom::Matrix3D,15,bool>",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "GFx",
            "AS3",
            "ThunkFunc0<Scaleform::GFx::AS3::Instances::fl_geom::Matrix3D,15,bool>"
        ]
    );
}

#[test]
fn test_type_462() {
    let result = parse_type(
        "struct __cppobj Scaleform::GFx::AS3::ThunkFunc0<Scaleform::GFx::AS3::Instances::fl_geom::Rectangle,8,Scaleform::GFx::AS3::SPtr<Scaleform::GFx::AS3::Instances::fl_geom::Point> >",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "GFx",
            "AS3",
            "ThunkFunc0<Scaleform::GFx::AS3::Instances::fl_geom::Rectangle,8,Scaleform::GFx::AS3::SPtr<Scaleform::GFx::AS3::Instances::fl_geom::Point> >"
        ]
    );
}

#[test]
fn test_type_463() {
    let result = parse_type(
        "struct __cppobj Scaleform::GFx::AS3::ThunkFunc0<Scaleform::GFx::AS3::Instances::fl_ui::ContextMenu,5,Scaleform::GFx::AS3::Value const >",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "GFx",
            "AS3",
            "ThunkFunc0<Scaleform::GFx::AS3::Instances::fl_ui::ContextMenu,5,Scaleform::GFx::AS3::Value const >"
        ]
    );
}

#[test]
fn test_type_464() {
    let result = parse_type(
        "struct __cppobj Scaleform::GFx::AS3::ThunkFunc1<Scaleform::GFx::AS3::Classes::fl_gfx::Extensions,0,Scaleform::GFx::AS3::Value const ,bool>",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "GFx",
            "AS3",
            "ThunkFunc1<Scaleform::GFx::AS3::Classes::fl_gfx::Extensions,0,Scaleform::GFx::AS3::Value const ,bool>"
        ]
    );
}

#[test]
fn test_type_465() {
    let result = parse_type(
        "struct __cppobj Scaleform::GFx::AS3::ThunkFunc1<Scaleform::GFx::AS3::Instances::fl_display::Sprite,1,Scaleform::GFx::AS3::Value const ,bool>",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "GFx",
            "AS3",
            "ThunkFunc1<Scaleform::GFx::AS3::Instances::fl_display::Sprite,1,Scaleform::GFx::AS3::Value const ,bool>"
        ]
    );
}

#[test]
fn test_type_466() {
    let result = parse_type(
        "struct __cppobj Scaleform::GFx::AS3::ThunkFunc1<Scaleform::GFx::AS3::Instances::fl_events::GeolocationEvent,7,Scaleform::GFx::AS3::Value const ,double>",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "GFx",
            "AS3",
            "ThunkFunc1<Scaleform::GFx::AS3::Instances::fl_events::GeolocationEvent,7,Scaleform::GFx::AS3::Value const ,double>"
        ]
    );
}

#[test]
fn test_type_467() {
    let result = parse_type(
        "struct __cppobj Scaleform::GFx::AS3::ThunkFunc1<Scaleform::GFx::AS3::Instances::fl_geom::Rectangle,13,Scaleform::GFx::AS3::Value const ,Scaleform::GFx::AS3::Instances::fl_geom::Point *>",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "GFx",
            "AS3",
            "ThunkFunc1<Scaleform::GFx::AS3::Instances::fl_geom::Rectangle,13,Scaleform::GFx::AS3::Value const ,Scaleform::GFx::AS3::Instances::fl_geom::Point *>"
        ]
    );
}

#[test]
fn test_type_468() {
    let result = parse_type(
        "struct __cppobj Scaleform::GFx::AS3::ThunkFunc1<Scaleform::GFx::AS3::Instances::fl_geom::Vector3D,6,Scaleform::GFx::AS3::Value const ,double>",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "GFx",
            "AS3",
            "ThunkFunc1<Scaleform::GFx::AS3::Instances::fl_geom::Vector3D,6,Scaleform::GFx::AS3::Value const ,double>"
        ]
    );
}

#[test]
fn test_type_469() {
    let result = parse_type(
        "struct __cppobj Scaleform::GFx::AS3::ThunkFunc1<Scaleform::GFx::AS3::Instances::fl_utils::Proxy,7,long,long>",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "GFx",
            "AS3",
            "ThunkFunc1<Scaleform::GFx::AS3::Instances::fl_utils::Proxy,7,long,long>"
        ]
    );
}

#[test]
fn test_type_470() {
    let result = parse_type(
        "struct __cppobj Scaleform::GFx::AS3::ThunkFunc2<Scaleform::GFx::AS3::Instances::fl_external::ExtensionContext,2,Scaleform::GFx::AS3::Value,unsigned int,Scaleform::GFx::AS3::Value *>",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "GFx",
            "AS3",
            "ThunkFunc2<Scaleform::GFx::AS3::Instances::fl_external::ExtensionContext,2,Scaleform::GFx::AS3::Value,unsigned int,Scaleform::GFx::AS3::Value *>"
        ]
    );
}

#[test]
fn test_type_471() {
    let result = parse_type(
        "struct __cppobj Scaleform::GFx::AS3::ThunkFunc2<Scaleform::GFx::AS3::Instances::fl::Date,33,Scaleform::GFx::AS3::Value,unsigned int,Scaleform::GFx::AS3::Value *>",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "GFx",
            "AS3",
            "ThunkFunc2<Scaleform::GFx::AS3::Instances::fl::Date,33,Scaleform::GFx::AS3::Value,unsigned int,Scaleform::GFx::AS3::Value *>"
        ]
    );
}

#[test]
fn test_type_472() {
    let result = parse_type(
        "struct __cppobj Scaleform::GFx::AS3::ThunkFunc3<Scaleform::GFx::AS3::Instances::fl_display::BitmapData,15,Scaleform::GFx::AS3::SPtr<Scaleform::GFx::AS3::Instances::fl_geom::Rectangle>,unsigned long,unsigned long,bool>",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "GFx",
            "AS3",
            "ThunkFunc3<Scaleform::GFx::AS3::Instances::fl_display::BitmapData,15,Scaleform::GFx::AS3::SPtr<Scaleform::GFx::AS3::Instances::fl_geom::Rectangle>,unsigned long,unsigned long,bool>"
        ]
    );
}

#[test]
fn test_type_473() {
    let result = parse_type(
        "struct __cppobj Scaleform::GFx::AS3::TR::InPSVisitor2<Scaleform::GFx::AS3::TR::InMarker,Scaleform::GFx::AS3::TR::DefSNodeVisitor2>",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "GFx",
            "AS3",
            "TR",
            "InPSVisitor2<Scaleform::GFx::AS3::TR::InMarker,Scaleform::GFx::AS3::TR::DefSNodeVisitor2>"
        ]
    );
}

#[test]
fn test_type_474() {
    let result = parse_type(
        "struct __cppobj Scaleform::GFx::AS3::UnboxArgV0<Scaleform::GFx::AS3::SPtr<Scaleform::GFx::AS3::Instances::fl_display::DisplayObjectContainer> >",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "GFx",
            "AS3",
            "UnboxArgV0<Scaleform::GFx::AS3::SPtr<Scaleform::GFx::AS3::Instances::fl_display::DisplayObjectContainer> >"
        ]
    );
}

#[test]
fn test_type_475() {
    let result = parse_type(
        "struct __cppobj Scaleform::GFx::AS3::UnboxArgV2NC<Scaleform::GFx::AS3::SPtr<Scaleform::GFx::AS3::Instances::fl_geom::Rectangle>,Scaleform::GFx::AS3::Instances::fl_geom::Rectangle *,Scaleform::GFx::AS3::Instances::fl_filters::BitmapFilter *> : Scaleform::GFx::AS3::UnboxArgV1NC<Scaleform::GFx::AS3::SPtr<Scaleform::GFx::AS3::Instances::fl_geom::Rectangle>,Scaleform::GFx::AS3::Instances::fl_geom::Rectangle *>",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "GFx",
            "AS3",
            "UnboxArgV2NC<Scaleform::GFx::AS3::SPtr<Scaleform::GFx::AS3::Instances::fl_geom::Rectangle>,Scaleform::GFx::AS3::Instances::fl_geom::Rectangle *,Scaleform::GFx::AS3::Instances::fl_filters::BitmapFilter *>"
        ]
    );
}

#[test]
fn test_type_476() {
    let result = parse_type(
        "struct __cppobj Scaleform::GFx::AS3::UnboxArgV3NC<Scaleform::GFx::AS3::Value const ,double,double,double> : Scaleform::GFx::AS3::UnboxArgV2NC<Scaleform::GFx::AS3::Value const ,double,double>",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "GFx",
            "AS3",
            "UnboxArgV3NC<Scaleform::GFx::AS3::Value const ,double,double,double>"
        ]
    );
}

#[test]
fn test_type_477() {
    let result = parse_type(
        "struct __cppobj Scaleform::GFx::AvmDisplayObjContainerBase : Scaleform::GFx::AvmInteractiveObjBase",
    );
    assert_eq!(
        result,
        vec!["Scaleform", "GFx", "AvmDisplayObjContainerBase"]
    );
}

#[test]
fn test_type_478() {
    let result =
        parse_type("struct __cppobj Scaleform::GFx::PtrGuard<Scaleform::Sound::SoundRenderer>");
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "GFx",
            "PtrGuard<Scaleform::Sound::SoundRenderer>"
        ]
    );
}

#[test]
fn test_type_479() {
    let result = parse_type(
        "struct __cppobj Scaleform::Hash<Scaleform::GFx::MovieDefImpl *,Scaleform::GFx::AS3::MovieRoot::LoadedMovieDefInfo,Scaleform::IdentityHash<Scaleform::GFx::MovieDefImpl *>,Scaleform::AllocatorLH<Scaleform::GFx::MovieDefImpl *,2>,Scaleform::HashNode<Scaleform::GFx::MovieDefImpl *,Scaleform::GFx::AS3::MovieRoot::LoadedMovieDefInfo,Scaleform::IdentityHash<Scaleform::GFx::MovieDefImpl *> >,Scaleform::HashsetNodeEntry<Scaleform::HashNode<Scaleform::GFx::MovieDefImpl *,Scaleform::GFx::AS3::MovieRoot::LoadedMovieDefInfo,Scaleform::IdentityHash<Scaleform::GFx::MovieDefImpl *> >,Scaleform::HashNode<Scaleform::GFx::MovieDefImpl *,Scaleform::GFx::AS3::MovieRoot::LoadedMovieDefInfo,Scaleform::IdentityHash<Scaleform::GFx::MovieDefImpl *> >::NodeHashF>,Scaleform::HashSet<Scaleform::HashNode<Scaleform::GFx::MovieDefImpl *,Scaleform::GFx::AS3::MovieRoot::LoadedMovieDefInfo,Scaleform::IdentityHash<Scaleform::GFx::MovieDefImpl *> >,Scaleform::HashNode<Scaleform::GFx::MovieDefImpl *,Scaleform::GFx::AS3::MovieRoot::LoadedMovieDefInfo,Scaleform::IdentityHash<Scaleform::GFx::MovieDefImpl *> >::NodeHashF,Scaleform::HashNode<Scaleform::GFx::MovieDefImpl *,Scaleform::GFx::AS3::MovieRoot::LoadedMovieDefInfo,Scaleform::IdentityHash<Scaleform::GFx::MovieDefImpl *> >::NodeAltHashF,Scaleform::AllocatorLH<Scaleform::GFx::MovieDefImpl *,2>,Scaleform::HashsetNodeEntry<Scaleform::HashNode<Scaleform::GFx::MovieDefImpl *,Scaleform::GFx::AS3::MovieRoot::LoadedMovieDefInfo,Scaleform::IdentityHash<Scaleform::GFx::MovieDefImpl *> >,Scaleform::HashNode<Scaleform::GFx::MovieDefImpl *,Scaleform::GFx::AS3::MovieRoot::LoadedMovieDefInfo,Scaleform::IdentityHash<Scaleform::GFx::MovieDefImpl *> >::NodeHashF> > >",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "Hash<Scaleform::GFx::MovieDefImpl *,Scaleform::GFx::AS3::MovieRoot::LoadedMovieDefInfo,Scaleform::IdentityHash<Scaleform::GFx::MovieDefImpl *>,Scaleform::AllocatorLH<Scaleform::GFx::MovieDefImpl *,2>,Scaleform::HashNode<Scaleform::GFx::MovieDefImpl *,Scaleform::GFx::AS3::MovieRoot::LoadedMovieDefInfo,Scaleform::IdentityHash<Scaleform::GFx::MovieDefImpl *> >,Scaleform::HashsetNodeEntry<Scaleform::HashNode<Scaleform::GFx::MovieDefImpl *,Scaleform::GFx::AS3::MovieRoot::LoadedMovieDefInfo,Scaleform::IdentityHash<Scaleform::GFx::MovieDefImpl *> >,Scaleform::HashNode<Scaleform::GFx::MovieDefImpl *,Scaleform::GFx::AS3::MovieRoot::LoadedMovieDefInfo,Scaleform::IdentityHash<Scaleform::GFx::MovieDefImpl *> >::NodeHashF>,Scaleform::HashSet<Scaleform::HashNode<Scaleform::GFx::MovieDefImpl *,Scaleform::GFx::AS3::MovieRoot::LoadedMovieDefInfo,Scaleform::IdentityHash<Scaleform::GFx::MovieDefImpl *> >,Scaleform::HashNode<Scaleform::GFx::MovieDefImpl *,Scaleform::GFx::AS3::MovieRoot::LoadedMovieDefInfo,Scaleform::IdentityHash<Scaleform::GFx::MovieDefImpl *> >::NodeHashF,Scaleform::HashNode<Scaleform::GFx::MovieDefImpl *,Scaleform::GFx::AS3::MovieRoot::LoadedMovieDefInfo,Scaleform::IdentityHash<Scaleform::GFx::MovieDefImpl *> >::NodeAltHashF,Scaleform::AllocatorLH<Scaleform::GFx::MovieDefImpl *,2>,Scaleform::HashsetNodeEntry<Scaleform::HashNode<Scaleform::GFx::MovieDefImpl *,Scaleform::GFx::AS3::MovieRoot::LoadedMovieDefInfo,Scaleform::IdentityHash<Scaleform::GFx::MovieDefImpl *> >,Scaleform::HashNode<Scaleform::GFx::MovieDefImpl *,Scaleform::GFx::AS3::MovieRoot::LoadedMovieDefInfo,Scaleform::IdentityHash<Scaleform::GFx::MovieDefImpl *> >::NodeHashF> > >"
        ]
    );
}

#[test]
fn test_type_480() {
    let result = parse_type(
        "struct __cppobj Scaleform::Hash<void const *,Scaleform::Ptr<ID3D11DeviceChild>,Scaleform::FixedSizeHash<void const *>,Scaleform::AllocatorLH<void const *,2>,Scaleform::HashNode<void const *,Scaleform::Ptr<ID3D11DeviceChild>,Scaleform::FixedSizeHash<void const *> >,Scaleform::HashsetCachedNodeEntry<Scaleform::HashNode<void const *,Scaleform::Ptr<ID3D11DeviceChild>,Scaleform::FixedSizeHash<void const *> >,Scaleform::HashNode<void const *,Scaleform::Ptr<ID3D11DeviceChild>,Scaleform::FixedSizeHash<void const *> >::NodeHashF>,Scaleform::HashSet<Scaleform::HashNode<void const *,Scaleform::Ptr<ID3D11DeviceChild>,Scaleform::FixedSizeHash<void const *> >,Scaleform::HashNode<void const *,Scaleform::Ptr<ID3D11DeviceChild>,Scaleform::FixedSizeHash<void const *> >::NodeHashF,Scaleform::HashNode<void const *,Scaleform::Ptr<ID3D11DeviceChild>,Scaleform::FixedSizeHash<void const *> >::NodeAltHashF,Scaleform::AllocatorLH<void const *,2>,Scaleform::HashsetCachedNodeEntry<Scaleform::HashNode<void const *,Scaleform::Ptr<ID3D11DeviceChild>,Scaleform::FixedSizeHash<void const *> >,Scaleform::HashNode<void const *,Scaleform::Ptr<ID3D11DeviceChild>,Scaleform::FixedSizeHash<void const *> >::NodeHashF> > >",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "Hash<void const *,Scaleform::Ptr<ID3D11DeviceChild>,Scaleform::FixedSizeHash<void const *>,Scaleform::AllocatorLH<void const *,2>,Scaleform::HashNode<void const *,Scaleform::Ptr<ID3D11DeviceChild>,Scaleform::FixedSizeHash<void const *> >,Scaleform::HashsetCachedNodeEntry<Scaleform::HashNode<void const *,Scaleform::Ptr<ID3D11DeviceChild>,Scaleform::FixedSizeHash<void const *> >,Scaleform::HashNode<void const *,Scaleform::Ptr<ID3D11DeviceChild>,Scaleform::FixedSizeHash<void const *> >::NodeHashF>,Scaleform::HashSet<Scaleform::HashNode<void const *,Scaleform::Ptr<ID3D11DeviceChild>,Scaleform::FixedSizeHash<void const *> >,Scaleform::HashNode<void const *,Scaleform::Ptr<ID3D11DeviceChild>,Scaleform::FixedSizeHash<void const *> >::NodeHashF,Scaleform::HashNode<void const *,Scaleform::Ptr<ID3D11DeviceChild>,Scaleform::FixedSizeHash<void const *> >::NodeAltHashF,Scaleform::AllocatorLH<void const *,2>,Scaleform::HashsetCachedNodeEntry<Scaleform::HashNode<void const *,Scaleform::Ptr<ID3D11DeviceChild>,Scaleform::FixedSizeHash<void const *> >,Scaleform::HashNode<void const *,Scaleform::Ptr<ID3D11DeviceChild>,Scaleform::FixedSizeHash<void const *> >::NodeHashF> > >"
        ]
    );
}

#[test]
fn test_type_481() {
    let result = parse_type(
        "struct __cppobj Scaleform::HashDH<Scaleform::GFx::AS3::TR::NodeBlock *,Scaleform::List<Scaleform::GFx::AS3::TR::Range,Scaleform::GFx::AS3::TR::Range,Scaleform::ListNode<Scaleform::GFx::AS3::TR::Range> > *,Scaleform::FixedSizeHash<Scaleform::GFx::AS3::TR::NodeBlock *>,2,Scaleform::HashNode<Scaleform::GFx::AS3::TR::NodeBlock *,Scaleform::List<Scaleform::GFx::AS3::TR::Range,Scaleform::GFx::AS3::TR::Range,Scaleform::ListNode<Scaleform::GFx::AS3::TR::Range> > *,Scaleform::FixedSizeHash<Scaleform::GFx::AS3::TR::NodeBlock *> >,Scaleform::HashsetCachedNodeEntry<Scaleform::HashNode<Scaleform::GFx::AS3::TR::NodeBlock *,Scaleform::List<Scaleform::GFx::AS3::TR::Range,Scaleform::GFx::AS3::TR::Range,Scaleform::ListNode<Scaleform::GFx::AS3::TR::Range> > *,Scaleform::FixedSizeHash<Scaleform::GFx::AS3::TR::NodeBlock *> >,Scaleform::HashNode<Scaleform::GFx::AS3::TR::NodeBlock *,Scaleform::List<Scaleform::GFx::AS3::TR::Range,Scaleform::GFx::AS3::TR::Range,Scaleform::ListNode<Scaleform::GFx::AS3::TR::Range> > *,Scaleform::FixedSizeHash<Scaleform::GFx::AS3::TR::NodeBlock *> >::NodeHashF> > : Scaleform::Hash<Scaleform::GFx::AS3::TR::NodeBlock *,Scaleform::List<Scaleform::GFx::AS3::TR::Range,Scaleform::GFx::AS3::TR::Range,Scaleform::ListNode<Scaleform::GFx::AS3::TR::Range> > *,Scaleform::FixedSizeHash<Scaleform::GFx::AS3::TR::NodeBlock *>,Scaleform::AllocatorDH<Scaleform::GFx::AS3::TR::NodeBlock *,2>,Scaleform::HashNode<Scaleform::GFx::AS3::TR::NodeBlock *,Scaleform::List<Scaleform::GFx::AS3::TR::Range,Scaleform::GFx::AS3::TR::Range,Scaleform::ListNode<Scaleform::GFx::AS3::TR::Range> > *,Scaleform::FixedSizeHash<Scaleform::GFx::AS3::TR::NodeBlock *> >,Scaleform::HashsetCachedNodeEntry<Scaleform::HashNode<Scaleform::GFx::AS3::TR::NodeBlock *,Scaleform::List<Scaleform::GFx::AS3::TR::Range,Scaleform::GFx::AS3::TR::Range,Scaleform::ListNode<Scaleform::GFx::AS3::TR::Range> > *,Scaleform::FixedSizeHash<Scaleform::GFx::AS3::TR::NodeBlock *> >,Scaleform::HashNode<Scaleform::GFx::AS3::TR::NodeBlock *,Scaleform::List<Scaleform::GFx::AS3::TR::Range,Scaleform::GFx::AS3::TR::Range,Scaleform::ListNode<Scaleform::GFx::AS3::TR::Range> > *,Scaleform::FixedSizeHash<Scaleform::GFx::AS3::TR::NodeBlock *> >::NodeHashF>,Scaleform::HashSetDH<Scaleform::HashNode<Scaleform::GFx::AS3::TR::NodeBlock *,Scaleform::List<Scaleform::GFx::AS3::TR::Range,Scaleform::GFx::AS3::TR::Range,Scaleform::ListNode<Scaleform::GFx::AS3::TR::Range> > *,Scaleform::FixedSizeHash<Scaleform::GFx::AS3::TR::NodeBlock *> >,Scaleform::HashNode<Scaleform::GFx::AS3::TR::NodeBlock *,Scaleform::List<Scaleform::GFx::AS3::TR::Range,Scaleform::GFx::AS3::TR::Range,Scaleform::ListNode<Scaleform::GFx::AS3::TR::Range> > *,Scaleform::FixedSizeHash<Scaleform::GFx::AS3::TR::NodeBlock *> >::NodeHashF,Scaleform::HashNode<Scaleform::GFx::AS3::TR::NodeBlock *,Scaleform::List<Scaleform::GFx::AS3::TR::Range,Scaleform::GFx::AS3::TR::Range,Scaleform::ListNode<Scaleform::GFx::AS3::TR::Range> > *,Scaleform::FixedSizeHash<Scaleform::GFx::AS3::TR::NodeBlock *> >::NodeAltHashF,2,Scaleform::HashsetCachedNodeEntry<Scaleform::HashNode<Scaleform::GFx::AS3::TR::NodeBlock *,Scaleform::List<Scaleform::GFx::AS3::TR::Range,Scaleform::GFx::AS3::TR::Range,Scaleform::ListNode<Scaleform::GFx::AS3::TR::Range> > *,Scaleform::FixedSizeHash<Scaleform::GFx::AS3::TR::NodeBlock *> >,Scaleform::HashNode<Scaleform::GFx::AS3::TR::NodeBlock *,Scaleform::List<Scaleform::GFx::AS3::TR::Range,Scaleform::GFx::AS3::TR::Range,Scaleform::ListNode<Scaleform::GFx::AS3::TR::Range> > *,Scaleform::FixedSizeHash<Scaleform::GFx::AS3::TR::NodeBlock *> >::NodeHashF> > >",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "HashDH<Scaleform::GFx::AS3::TR::NodeBlock *,Scaleform::List<Scaleform::GFx::AS3::TR::Range,Scaleform::GFx::AS3::TR::Range,Scaleform::ListNode<Scaleform::GFx::AS3::TR::Range> > *,Scaleform::FixedSizeHash<Scaleform::GFx::AS3::TR::NodeBlock *>,2,Scaleform::HashNode<Scaleform::GFx::AS3::TR::NodeBlock *,Scaleform::List<Scaleform::GFx::AS3::TR::Range,Scaleform::GFx::AS3::TR::Range,Scaleform::ListNode<Scaleform::GFx::AS3::TR::Range> > *,Scaleform::FixedSizeHash<Scaleform::GFx::AS3::TR::NodeBlock *> >,Scaleform::HashsetCachedNodeEntry<Scaleform::HashNode<Scaleform::GFx::AS3::TR::NodeBlock *,Scaleform::List<Scaleform::GFx::AS3::TR::Range,Scaleform::GFx::AS3::TR::Range,Scaleform::ListNode<Scaleform::GFx::AS3::TR::Range> > *,Scaleform::FixedSizeHash<Scaleform::GFx::AS3::TR::NodeBlock *> >,Scaleform::HashNode<Scaleform::GFx::AS3::TR::NodeBlock *,Scaleform::List<Scaleform::GFx::AS3::TR::Range,Scaleform::GFx::AS3::TR::Range,Scaleform::ListNode<Scaleform::GFx::AS3::TR::Range> > *,Scaleform::FixedSizeHash<Scaleform::GFx::AS3::TR::NodeBlock *> >::NodeHashF> >"
        ]
    );
}

#[test]
fn test_type_482() {
    let result = parse_type(
        "struct __cppobj Scaleform::HashNode<Scaleform::GFx::AS3::Value,Scaleform::GFx::AS3::Value,Scaleform::GFx::AS3::Value::HashFunctor>::NodeRef",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "HashNode<Scaleform::GFx::AS3::Value,Scaleform::GFx::AS3::Value,Scaleform::GFx::AS3::Value::HashFunctor>",
            "NodeRef"
        ]
    );
}

#[test]
fn test_type_483() {
    let result = parse_type(
        "struct __cppobj Scaleform::HashNode<Scaleform::GFx::Text::StyleKey,Scaleform::Render::Text::Style *,Scaleform::GFx::Text::StyleHashFunc<Scaleform::GFx::Text::StyleKey> >",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "HashNode<Scaleform::GFx::Text::StyleKey,Scaleform::Render::Text::Style *,Scaleform::GFx::Text::StyleHashFunc<Scaleform::GFx::Text::StyleKey> >"
        ]
    );
}

#[test]
fn test_type_484() {
    let result = parse_type(
        "struct __cppobj Scaleform::HashNode<unsigned __int64,Scaleform::Ptr<Scaleform::GFx::AMP::FunctionDesc>,Scaleform::FixedSizeHash<unsigned __int64> >::NodeHashF",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "HashNode<unsigned __int64,Scaleform::Ptr<Scaleform::GFx::AMP::FunctionDesc>,Scaleform::FixedSizeHash<unsigned __int64> >",
            "NodeHashF"
        ]
    );
}

#[test]
fn test_type_485() {
    let result = parse_type(
        "struct __cppobj Scaleform::HashNode<unsigned long,Scaleform::GFx::AS3::TypeBarrier *,Scaleform::FixedSizeHash<unsigned long> >::NodeAltHashF",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "HashNode<unsigned long,Scaleform::GFx::AS3::TypeBarrier *,Scaleform::FixedSizeHash<unsigned long> >",
            "NodeAltHashF"
        ]
    );
}

#[test]
fn test_type_486() {
    let result = parse_type(
        "struct __cppobj Scaleform::HashSet<unsigned int,Scaleform::FixedSizeHash<unsigned int>,Scaleform::FixedSizeHash<unsigned int>,Scaleform::AllocatorLH<unsigned int,2>,Scaleform::HashsetCachedEntry<unsigned int,Scaleform::FixedSizeHash<unsigned int> > > : Scaleform::HashSetBase<unsigned int,Scaleform::FixedSizeHash<unsigned int>,Scaleform::FixedSizeHash<unsigned int>,Scaleform::AllocatorLH<unsigned int,2>,Scaleform::HashsetCachedEntry<unsigned int,Scaleform::FixedSizeHash<unsigned int> > >",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "HashSet<unsigned int,Scaleform::FixedSizeHash<unsigned int>,Scaleform::FixedSizeHash<unsigned int>,Scaleform::AllocatorLH<unsigned int,2>,Scaleform::HashsetCachedEntry<unsigned int,Scaleform::FixedSizeHash<unsigned int> > >"
        ]
    );
}

#[test]
fn test_type_487() {
    let result = parse_type(
        "struct __cppobj Scaleform::HashSetBase<Scaleform::HashNode<Scaleform::GFx::AS3::Abc::MbiInd,Scaleform::GFx::AS3::SPtr<Scaleform::GFx::AS3::InstanceTraits::Traits>,Scaleform::FixedSizeHash<Scaleform::GFx::AS3::Abc::MbiInd> >,Scaleform::HashNode<Scaleform::GFx::AS3::Abc::MbiInd,Scaleform::GFx::AS3::SPtr<Scaleform::GFx::AS3::InstanceTraits::Traits>,Scaleform::FixedSizeHash<Scaleform::GFx::AS3::Abc::MbiInd> >::NodeHashF,Scaleform::HashNode<Scaleform::GFx::AS3::Abc::MbiInd,Scaleform::GFx::AS3::SPtr<Scaleform::GFx::AS3::InstanceTraits::Traits>,Scaleform::FixedSizeHash<Scaleform::GFx::AS3::Abc::MbiInd> >::NodeAltHashF,Scaleform::AllocatorLH<Scaleform::GFx::AS3::Abc::MbiInd,341>,Scaleform::HashsetCachedNodeEntry<Scaleform::HashNode<Scaleform::GFx::AS3::Abc::MbiInd,Scaleform::GFx::AS3::SPtr<Scaleform::GFx::AS3::InstanceTraits::Traits>,Scaleform::FixedSizeHash<Scaleform::GFx::AS3::Abc::MbiInd> >,Scaleform::HashNode<Scaleform::GFx::AS3::Abc::MbiInd,Scaleform::GFx::AS3::SPtr<Scaleform::GFx::AS3::InstanceTraits::Traits>,Scaleform::FixedSizeHash<Scaleform::GFx::AS3::Abc::MbiInd> >::NodeHashF> >::ConstIterator",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "HashSetBase<Scaleform::HashNode<Scaleform::GFx::AS3::Abc::MbiInd,Scaleform::GFx::AS3::SPtr<Scaleform::GFx::AS3::InstanceTraits::Traits>,Scaleform::FixedSizeHash<Scaleform::GFx::AS3::Abc::MbiInd> >,Scaleform::HashNode<Scaleform::GFx::AS3::Abc::MbiInd,Scaleform::GFx::AS3::SPtr<Scaleform::GFx::AS3::InstanceTraits::Traits>,Scaleform::FixedSizeHash<Scaleform::GFx::AS3::Abc::MbiInd> >::NodeHashF,Scaleform::HashNode<Scaleform::GFx::AS3::Abc::MbiInd,Scaleform::GFx::AS3::SPtr<Scaleform::GFx::AS3::InstanceTraits::Traits>,Scaleform::FixedSizeHash<Scaleform::GFx::AS3::Abc::MbiInd> >::NodeAltHashF,Scaleform::AllocatorLH<Scaleform::GFx::AS3::Abc::MbiInd,341>,Scaleform::HashsetCachedNodeEntry<Scaleform::HashNode<Scaleform::GFx::AS3::Abc::MbiInd,Scaleform::GFx::AS3::SPtr<Scaleform::GFx::AS3::InstanceTraits::Traits>,Scaleform::FixedSizeHash<Scaleform::GFx::AS3::Abc::MbiInd> >,Scaleform::HashNode<Scaleform::GFx::AS3::Abc::MbiInd,Scaleform::GFx::AS3::SPtr<Scaleform::GFx::AS3::InstanceTraits::Traits>,Scaleform::FixedSizeHash<Scaleform::GFx::AS3::Abc::MbiInd> >::NodeHashF> >",
            "ConstIterator"
        ]
    );
}

#[test]
fn test_type_488() {
    let result = parse_type(
        "struct __cppobj Scaleform::HashSetBase<Scaleform::HashNode<Scaleform::String,Scaleform::StatsUpdate::FileStats,Scaleform::String::NoCaseHashFunctor>,Scaleform::HashNode<Scaleform::String,Scaleform::StatsUpdate::FileStats,Scaleform::String::NoCaseHashFunctor>::NodeHashF,Scaleform::HashNode<Scaleform::String,Scaleform::StatsUpdate::FileStats,Scaleform::String::NoCaseHashFunctor>::NodeAltHashF,Scaleform::AllocatorGH<Scaleform::StatsUpdate::FileStats,2>,Scaleform::HashsetCachedNodeEntry<Scaleform::HashNode<Scaleform::String,Scaleform::StatsUpdate::FileStats,Scaleform::String::NoCaseHashFunctor>,Scaleform::HashNode<Scaleform::String,Scaleform::StatsUpdate::FileStats,Scaleform::String::NoCaseHashFunctor>::NodeHashF> >",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "HashSetBase<Scaleform::HashNode<Scaleform::String,Scaleform::StatsUpdate::FileStats,Scaleform::String::NoCaseHashFunctor>,Scaleform::HashNode<Scaleform::String,Scaleform::StatsUpdate::FileStats,Scaleform::String::NoCaseHashFunctor>::NodeHashF,Scaleform::HashNode<Scaleform::String,Scaleform::StatsUpdate::FileStats,Scaleform::String::NoCaseHashFunctor>::NodeAltHashF,Scaleform::AllocatorGH<Scaleform::StatsUpdate::FileStats,2>,Scaleform::HashsetCachedNodeEntry<Scaleform::HashNode<Scaleform::String,Scaleform::StatsUpdate::FileStats,Scaleform::String::NoCaseHashFunctor>,Scaleform::HashNode<Scaleform::String,Scaleform::StatsUpdate::FileStats,Scaleform::String::NoCaseHashFunctor>::NodeHashF> >"
        ]
    );
}

#[test]
fn test_type_489() {
    let result = parse_type(
        "struct __cppobj Scaleform::HashsetCachedEntry<Scaleform::Ptr<Scaleform::Render::VectorGlyphShape>,Scaleform::Render::VectorGlyphShape::PtrHashFunctor>",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "HashsetCachedEntry<Scaleform::Ptr<Scaleform::Render::VectorGlyphShape>,Scaleform::Render::VectorGlyphShape::PtrHashFunctor>"
        ]
    );
}

#[test]
fn test_type_490() {
    let result = parse_type(
        "struct __cppobj Scaleform::HashSetDH<Scaleform::HashNode<unsigned __int64,Scaleform::GFx::AS3::Value,Scaleform::FixedSizeHash<unsigned __int64> >,Scaleform::HashNode<unsigned __int64,Scaleform::GFx::AS3::Value,Scaleform::FixedSizeHash<unsigned __int64> >::NodeHashF,Scaleform::HashNode<unsigned __int64,Scaleform::GFx::AS3::Value,Scaleform::FixedSizeHash<unsigned __int64> >::NodeAltHashF,2,Scaleform::HashsetCachedNodeEntry<Scaleform::HashNode<unsigned __int64,Scaleform::GFx::AS3::Value,Scaleform::FixedSizeHash<unsigned __int64> >,Scaleform::HashNode<unsigned __int64,Scaleform::GFx::AS3::Value,Scaleform::FixedSizeHash<unsigned __int64> >::NodeHashF> > : Scaleform::HashSetBase<Scaleform::HashNode<unsigned __int64,Scaleform::GFx::AS3::Value,Scaleform::FixedSizeHash<unsigned __int64> >,Scaleform::HashNode<unsigned __int64,Scaleform::GFx::AS3::Value,Scaleform::FixedSizeHash<unsigned __int64> >::NodeHashF,Scaleform::HashNode<unsigned __int64,Scaleform::GFx::AS3::Value,Scaleform::FixedSizeHash<unsigned __int64> >::NodeAltHashF,Scaleform::AllocatorDH<Scaleform::HashNode<unsigned __int64,Scaleform::GFx::AS3::Value,Scaleform::FixedSizeHash<unsigned __int64> >,2>,Scaleform::HashsetCachedNodeEntry<Scaleform::HashNode<unsigned __int64,Scaleform::GFx::AS3::Value,Scaleform::FixedSizeHash<unsigned __int64> >,Scaleform::HashNode<unsigned __int64,Scaleform::GFx::AS3::Value,Scaleform::FixedSizeHash<unsigned __int64> >::NodeHashF> >",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "HashSetDH<Scaleform::HashNode<unsigned __int64,Scaleform::GFx::AS3::Value,Scaleform::FixedSizeHash<unsigned __int64> >,Scaleform::HashNode<unsigned __int64,Scaleform::GFx::AS3::Value,Scaleform::FixedSizeHash<unsigned __int64> >::NodeHashF,Scaleform::HashNode<unsigned __int64,Scaleform::GFx::AS3::Value,Scaleform::FixedSizeHash<unsigned __int64> >::NodeAltHashF,2,Scaleform::HashsetCachedNodeEntry<Scaleform::HashNode<unsigned __int64,Scaleform::GFx::AS3::Value,Scaleform::FixedSizeHash<unsigned __int64> >,Scaleform::HashNode<unsigned __int64,Scaleform::GFx::AS3::Value,Scaleform::FixedSizeHash<unsigned __int64> >::NodeHashF> >"
        ]
    );
}

#[test]
fn test_type_491() {
    let result = parse_type(
        "struct __cppobj Scaleform::HashSetUncached<Scaleform::GFx::Resource *,Scaleform::GFx::ResourceLib::ResourcePtrHashFunc,Scaleform::GFx::ResourceLib::ResourcePtrHashFunc,Scaleform::AllocatorGH<Scaleform::GFx::Resource *,2> > : Scaleform::HashSet<Scaleform::GFx::Resource *,Scaleform::GFx::ResourceLib::ResourcePtrHashFunc,Scaleform::GFx::ResourceLib::ResourcePtrHashFunc,Scaleform::AllocatorGH<Scaleform::GFx::Resource *,2>,Scaleform::HashsetEntry<Scaleform::GFx::Resource *,Scaleform::GFx::ResourceLib::ResourcePtrHashFunc> >",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "HashSetUncached<Scaleform::GFx::Resource *,Scaleform::GFx::ResourceLib::ResourcePtrHashFunc,Scaleform::GFx::ResourceLib::ResourcePtrHashFunc,Scaleform::AllocatorGH<Scaleform::GFx::Resource *,2> >"
        ]
    );
}

#[test]
fn test_type_492() {
    let result = parse_type(
        "struct __cppobj Scaleform::HashUncachedLH<enum Scaleform::GFx::AS2::ASBuiltinType,Scaleform::Ptr<Scaleform::GFx::AS2::Object>,Scaleform::FixedSizeHash<enum Scaleform::GFx::AS2::ASBuiltinType>,2> : Scaleform::HashLH<enum Scaleform::GFx::AS2::ASBuiltinType,Scaleform::Ptr<Scaleform::GFx::AS2::Object>,Scaleform::FixedSizeHash<enum Scaleform::GFx::AS2::ASBuiltinType>,2,Scaleform::HashNode<enum Scaleform::GFx::AS2::ASBuiltinType,Scaleform::Ptr<Scaleform::GFx::AS2::Object>,Scaleform::FixedSizeHash<enum Scaleform::GFx::AS2::ASBuiltinType> >,Scaleform::HashsetNodeEntry<Scaleform::HashNode<enum Scaleform::GFx::AS2::ASBuiltinType,Scaleform::Ptr<Scaleform::GFx::AS2::Object>,Scaleform::FixedSizeHash<enum Scaleform::GFx::AS2::ASBuiltinType> >,Scaleform::HashNode<enum Scaleform::GFx::AS2::ASBuiltinType,Scaleform::Ptr<Scaleform::GFx::AS2::Object>,Scaleform::FixedSizeHash<enum Scaleform::GFx::AS2::ASBuiltinType> >::NodeHashF> >",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "HashUncachedLH<enum Scaleform::GFx::AS2::ASBuiltinType,Scaleform::Ptr<Scaleform::GFx::AS2::Object>,Scaleform::FixedSizeHash<enum Scaleform::GFx::AS2::ASBuiltinType>,2>"
        ]
    );
}

#[test]
fn test_type_493() {
    let result = parse_type(
        "struct __cppobj Scaleform::ListNode<Scaleform::Render::ContextImpl::Entry::PropagateNode>",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "ListNode<Scaleform::Render::ContextImpl::Entry::PropagateNode>"
        ]
    );
}

#[test]
fn test_type_494() {
    let result = parse_type("struct __cppobj Scaleform::ListNode<Scaleform::Render::GlyphSlot>");
    assert_eq!(
        result,
        vec!["Scaleform", "ListNode<Scaleform::Render::GlyphSlot>"]
    );
}

#[test]
fn test_type_495() {
    let result = parse_type("struct __cppobj Scaleform::Pickable<Scaleform::GFx::AS3::Class>");
    assert_eq!(
        result,
        vec!["Scaleform", "Pickable<Scaleform::GFx::AS3::Class>"]
    );
}

#[test]
fn test_type_496() {
    let result = parse_type(
        "struct __cppobj Scaleform::Pickable<Scaleform::GFx::AS3::Instances::fl_display::GraphicsBitmapFill>",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "Pickable<Scaleform::GFx::AS3::Instances::fl_display::GraphicsBitmapFill>"
        ]
    );
}

#[test]
fn test_type_497() {
    let result = parse_type(
        "struct __cppobj Scaleform::Pickable<Scaleform::GFx::AS3::Instances::fl_events::StatusEvent>",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "Pickable<Scaleform::GFx::AS3::Instances::fl_events::StatusEvent>"
        ]
    );
}

#[test]
fn test_type_498() {
    let result =
        parse_type("struct __cppobj Scaleform::Pickable<Scaleform::GFx::ASIntervalTimerIntf>");
    assert_eq!(
        result,
        vec!["Scaleform", "Pickable<Scaleform::GFx::ASIntervalTimerIntf>"]
    );
}

#[test]
fn test_type_499() {
    let result = parse_type("struct __cppobj Scaleform::Pickable<Scaleform::GFx::StaticTextDef>");
    assert_eq!(
        result,
        vec!["Scaleform", "Pickable<Scaleform::GFx::StaticTextDef>"]
    );
}

#[test]
fn test_type_500() {
    let result =
        parse_type("struct __cppobj Scaleform::Pickable<Scaleform::Render::MeshKeyManager>");
    assert_eq!(
        result,
        vec!["Scaleform", "Pickable<Scaleform::Render::MeshKeyManager>"]
    );
}

#[test]
fn test_type_501() {
    let result = parse_type("struct __cppobj Scaleform::Ptr<Scaleform::GFx::FSCommandHandler>");
    assert_eq!(
        result,
        vec!["Scaleform", "Ptr<Scaleform::GFx::FSCommandHandler>"]
    );
}

#[test]
fn test_type_502() {
    let result = parse_type("struct __cppobj Scaleform::Ptr<Scaleform::Render::BlendPrimitive>");
    assert_eq!(
        result,
        vec!["Scaleform", "Ptr<Scaleform::Render::BlendPrimitive>"]
    );
}

#[test]
fn test_type_503() {
    let result =
        parse_type("struct __cppobj Scaleform::Ptr<Scaleform::Render::Text::ParagraphFormat>");
    assert_eq!(
        result,
        vec!["Scaleform", "Ptr<Scaleform::Render::Text::ParagraphFormat>"]
    );
}

#[test]
fn test_type_504() {
    let result = parse_type(
        "struct __cppobj Scaleform::RefCountBase<Scaleform::Render::ViewMatrix3DPrimitive,67> : Scaleform::RefCountBaseStatImpl<Scaleform::RefCountImpl,67>",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "RefCountBase<Scaleform::Render::ViewMatrix3DPrimitive,67>"
        ]
    );
}

#[test]
fn test_type_505() {
    let result = parse_type(
        "struct __cppobj Scaleform::RefCountBaseStatImpl<Scaleform::RefCountImpl,73> : Scaleform::RefCountImpl",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "RefCountBaseStatImpl<Scaleform::RefCountImpl,73>"
        ]
    );
}

#[test]
fn test_type_506() {
    let result = parse_type(
        "struct __cppobj Scaleform::Render::ContextImpl::ContextData_ImplMixin<Scaleform::Render::TreeShape::NodeData,Scaleform::Render::TreeNode::NodeData> : Scaleform::Render::TreeNode::NodeData",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "Render",
            "ContextImpl",
            "ContextData_ImplMixin<Scaleform::Render::TreeShape::NodeData,Scaleform::Render::TreeNode::NodeData>"
        ]
    );
}

#[test]
fn test_type_507() {
    let result = parse_type(
        "struct __cppobj Scaleform::Render::DICommand_CreateTexture : Scaleform::Render::DICommandImpl<Scaleform::Render::DICommand_CreateTexture,Scaleform::Render::DICommand>",
    );
    assert_eq!(
        result,
        vec!["Scaleform", "Render", "DICommand_CreateTexture"]
    );
}

#[test]
fn test_type_508() {
    let result = parse_type(
        "struct __cppobj Scaleform::Render::HALBeginDisplayItem : Scaleform::Render::RenderQueueItem::Interface",
    );
    assert_eq!(result, vec!["Scaleform", "Render", "HALBeginDisplayItem"]);
}

#[test]
fn test_type_509() {
    let result = parse_type("struct __cppobj Scaleform::Render::MeshKeySetHandle");
    assert_eq!(result, vec!["Scaleform", "Render", "MeshKeySetHandle"]);
}

#[test]
fn test_type_510() {
    let result = parse_type(
        "struct __cppobj Scaleform::Render::Rect<double> : Scaleform::Render::RectData<double>",
    );
    assert_eq!(result, vec!["Scaleform", "Render", "Rect<double>"]);
}

#[test]
fn test_type_511() {
    let result = parse_type(
        "struct __cppobj Scaleform::Render::StateData::Interface_Value : Scaleform::Render::StateData::Interface",
    );
    assert_eq!(
        result,
        vec!["Scaleform", "Render", "StateData", "Interface_Value"]
    );
}

#[test]
fn test_type_512() {
    let result = parse_type(
        "struct __cppobj Scaleform::Render::Text::DocView::DocumentText : Scaleform::Render::Text::StyledText",
    );
    assert_eq!(
        result,
        vec!["Scaleform", "Render", "Text", "DocView", "DocumentText"]
    );
}

#[test]
fn test_type_513() {
    let result = parse_type(
        "struct __cppobj Scaleform::Render::Text::PtrCompare<Scaleform::Render::Text::ParagraphFormat *>",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "Render",
            "Text",
            "PtrCompare<Scaleform::Render::Text::ParagraphFormat *>"
        ]
    );
}

#[test]
fn test_type_514() {
    let result = parse_type("struct __cppobj Scaleform::Render::Text::TextFormat::HashFunctor");
    assert_eq!(
        result,
        vec!["Scaleform", "Render", "Text", "TextFormat", "HashFunctor"]
    );
}

#[test]
fn test_type_515() {
    let result = parse_type(
        "struct __cppobj Scaleform::Render::WrapperImageSource : Scaleform::Render::ImageSource",
    );
    assert_eq!(result, vec!["Scaleform", "Render", "WrapperImageSource"]);
}

#[test]
fn test_type_516() {
    let result = parse_type("struct __cppobj Scaleform::ThreadCheckBase");
    assert_eq!(result, vec!["Scaleform", "ThreadCheckBase"]);
}

#[test]
fn test_type_517() {
    let result = parse_type("struct __cppobj SExternalScriptVariable");
    assert_eq!(result, vec!["SExternalScriptVariable"]);
}

#[test]
fn test_type_518() {
    let result = parse_type("struct __cppobj SRoadGraphPathfindRequest");
    assert_eq!(result, vec!["SRoadGraphPathfindRequest"]);
}

#[test]
fn test_type_519() {
    let result = parse_type(
        "struct __cppobj std::_Allocator_base<boost::lockfree::stack<Graphics::HConstantBuffer_t *,boost::parameter::void_,boost::parameter::void_,boost::parameter::void_>::node>",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "_Allocator_base<boost::lockfree::stack<Graphics::HConstantBuffer_t *,boost::parameter::void_,boost::parameter::void_,boost::parameter::void_>::node>"
        ]
    );
}

#[test]
fn test_type_520() {
    let result =
        parse_type("struct __cppobj std::_Allocator_base<boost::shared_ptr<SFMODBankInfo> >");
    assert_eq!(
        result,
        vec!["std", "_Allocator_base<boost::shared_ptr<SFMODBankInfo> >"]
    );
}

#[test]
fn test_type_521() {
    let result = parse_type(
        "struct __cppobj std::_Allocator_base<CAirplaneFlybyWeaponComponent::SSpawnedVehicle>",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "_Allocator_base<CAirplaneFlybyWeaponComponent::SSpawnedVehicle>"
        ]
    );
}

#[test]
fn test_type_522() {
    let result = parse_type("struct __cppobj std::_Allocator_base<CClimateZone::STextureZone>");
    assert_eq!(
        result,
        vec!["std", "_Allocator_base<CClimateZone::STextureZone>"]
    );
}

#[test]
fn test_type_523() {
    let result = parse_type(
        "struct __cppobj std::_Allocator_base<CGrapplingHookPropData::SInstanceState::SAttachment>",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "_Allocator_base<CGrapplingHookPropData::SInstanceState::SAttachment>"
        ]
    );
}

#[test]
fn test_type_524() {
    let result = parse_type("struct __cppobj std::_Allocator_base<CHUDUI::SActiveExplosiveHint>");
    assert_eq!(
        result,
        vec!["std", "_Allocator_base<CHUDUI::SActiveExplosiveHint>"]
    );
}

#[test]
fn test_type_525() {
    let result = parse_type("struct __cppobj std::_Allocator_base<CRoadMeshManager::SRoad>");
    assert_eq!(
        result,
        vec!["std", "_Allocator_base<CRoadMeshManager::SRoad>"]
    );
}

#[test]
fn test_type_526() {
    let result = parse_type("struct __cppobj std::_Allocator_base<CVocalsBase::SRecievePair>");
    assert_eq!(
        result,
        vec!["std", "_Allocator_base<CVocalsBase::SRecievePair>"]
    );
}

#[test]
fn test_type_527() {
    let result = parse_type("struct __cppobj std::_Allocator_base<SEventID>");
    assert_eq!(result, vec!["std", "_Allocator_base<SEventID>"]);
}

#[test]
fn test_type_528() {
    let result = parse_type(
        "struct __cppobj std::_Allocator_base<std::_List_nod<std::pair<NRevolution::SProvinceRecord const * const,std::string >>::_Node>",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "_Allocator_base<std::_List_nod<std::pair<NRevolution::SProvinceRecord const * const,std::string >>::_Node>"
        ]
    );
}

#[test]
fn test_type_529() {
    let result = parse_type(
        "struct __cppobj std::_Allocator_base<std::_Tree_nod<std::_Tmap_traits<ILogListener *,enum CLog::ELevel,std::less<ILogListener *>,std::allocator<std::pair<ILogListener * const,enum CLog::ELevel> >,0> >::_Node>",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "_Allocator_base<std::_Tree_nod<std::_Tmap_traits<ILogListener *,enum CLog::ELevel,std::less<ILogListener *>,std::allocator<std::pair<ILogListener * const,enum CLog::ELevel> >,0> >::_Node>"
        ]
    );
}

#[test]
fn test_type_530() {
    let result = parse_type(
        "struct __cppobj std::_Allocator_base<std::_Tree_nod<std::_Tmap_traits<std::string,std::string,std::less<std::string >,std::allocator<std::pair<std::string const ,std::string > >,0> >::_Node>",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "_Allocator_base<std::_Tree_nod<std::_Tmap_traits<std::string,std::string,std::less<std::string >,std::allocator<std::pair<std::string const ,std::string > >,0> >::_Node>"
        ]
    );
}

#[test]
fn test_type_531() {
    let result = parse_type(
        "struct __cppobj std::_Allocator_base<std::_Tree_nod<std::_Tmap_traits<void *,char const *,std::less<void *>,std::allocator<std::pair<void * const,char const *> >,0> >::_Node>",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "_Allocator_base<std::_Tree_nod<std::_Tmap_traits<void *,char const *,std::less<void *>,std::allocator<std::pair<void * const,char const *> >,0> >::_Node>"
        ]
    );
}

#[test]
fn test_type_532() {
    let result = parse_type(
        "struct __cppobj std::_Allocator_base<std::_Tree_nod<std::_Tset_traits<testing::internal::String,std::less<testing::internal::String>,std::allocator<testing::internal::String>,0> >::_Node>",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "_Allocator_base<std::_Tree_nod<std::_Tset_traits<testing::internal::String,std::less<testing::internal::String>,std::allocator<testing::internal::String>,0> >::_Node>"
        ]
    );
}

#[test]
fn test_type_533() {
    let result = parse_type(
        "struct __cppobj std::_Allocator_base<std::pair<CROMUI::StatProp,CStatisticContext> >",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "_Allocator_base<std::pair<CROMUI::StatProp,CStatisticContext> >"
        ]
    );
}

#[test]
fn test_type_534() {
    let result = parse_type(
        "struct __cppobj std::_Allocator_base<std::tr1::_Function_impl1<void,CHashString const &> >",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "_Allocator_base<std::tr1::_Function_impl1<void,CHashString const &> >"
        ]
    );
}

#[test]
fn test_type_535() {
    let result = parse_type(
        "struct __cppobj std::_Allocator_base<std::tr1::_Impl_no_alloc0<std::tr1::_Callable_obj<NPfxBreakable::SDeferredImpulseApplicator_World<CPfxBreakable,2740202110>,0>,void> >",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "_Allocator_base<std::tr1::_Impl_no_alloc0<std::tr1::_Callable_obj<NPfxBreakable::SDeferredImpulseApplicator_World<CPfxBreakable,2740202110>,0>,void> >"
        ]
    );
}

#[test]
fn test_type_536() {
    let result = parse_type(
        "struct __cppobj std::_Allocator_base<std::tr1::_Impl_no_alloc1<std::tr1::_Callable_fun<void (__cdecl*const)(float const *),0>,void,float const *> >",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "_Allocator_base<std::tr1::_Impl_no_alloc1<std::tr1::_Callable_fun<void (__cdecl*const)(float const *),0>,void,float const *> >"
        ]
    );
}

#[test]
fn test_type_537() {
    let result = parse_type(
        "struct __cppobj std::_Allocator_base<std::tr1::_Impl_no_alloc1<std::tr1::_Callable_obj<<lambda111>,0>,void,CLeaderboardStrategy<CAsynchronousBragEntry<float>,float>::FetchResult> >",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "_Allocator_base<std::tr1::_Impl_no_alloc1<std::tr1::_Callable_obj<<lambda111>,0>,void,CLeaderboardStrategy<CAsynchronousBragEntry<float>,float>::FetchResult> >"
        ]
    );
}

#[test]
fn test_type_538() {
    let result = parse_type(
        "struct __cppobj std::_Allocator_base<std::tr1::_Impl_no_alloc1<std::tr1::_Callable_obj<<lambda179>,0>,void,CCallContext &> >",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "_Allocator_base<std::tr1::_Impl_no_alloc1<std::tr1::_Callable_obj<<lambda179>,0>,void,CCallContext &> >"
        ]
    );
}

#[test]
fn test_type_539() {
    let result = parse_type(
        "struct __cppobj std::_Allocator_base<std::tr1::_Impl_no_alloc1<std::tr1::_Callable_obj<<lambda83>,0>,void,Scaleform::GFx::Value &> >",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "_Allocator_base<std::tr1::_Impl_no_alloc1<std::tr1::_Callable_obj<<lambda83>,0>,void,Scaleform::GFx::Value &> >"
        ]
    );
}

#[test]
fn test_type_540() {
    let result = parse_type(
        "struct __cppobj std::_Allocator_base<std::tr1::_Impl_no_alloc2<std::tr1::_Callable_obj<<lambda28>,0>,void,CLeaderboardPage<SLeaderboardEntry<__int64>,__int64> &,enum NOnline::EErrorCode> >",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "_Allocator_base<std::tr1::_Impl_no_alloc2<std::tr1::_Callable_obj<<lambda28>,0>,void,CLeaderboardPage<SLeaderboardEntry<__int64>,__int64> &,enum NOnline::EErrorCode> >"
        ]
    );
}

#[test]
fn test_type_541() {
    let result = parse_type(
        "struct __cppobj std::_Allocator_base<std::unique_ptr<Input::SDeviceObjectInfo> >",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "_Allocator_base<std::unique_ptr<Input::SDeviceObjectInfo> >"
        ]
    );
}

#[test]
fn test_type_542() {
    let result = parse_type(
        "struct __cppobj std::_Allocator_base<std::unique_ptr<NGraphScript::CNodeOnEventEventHandler> >",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "_Allocator_base<std::unique_ptr<NGraphScript::CNodeOnEventEventHandler> >"
        ]
    );
}

#[test]
fn test_type_543() {
    let result = parse_type("struct __cppobj std::_Ctraits<float>");
    assert_eq!(result, vec!["std", "_Ctraits<float>"]);
}

#[test]
fn test_type_544() {
    let result = parse_type(
        "struct __cppobj std::_Iterator012<std::bidirectional_iterator_tag,CLeaderboardPage<CAsynchronousBragEntry<__int64>,__int64>,__int64,CLeaderboardPage<CAsynchronousBragEntry<__int64>,__int64> const *,CLeaderboardPage<CAsynchronousBragEntry<__int64>,__int64> const &,std::_Iterator_base0> : std::_Iterator_base0",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "_Iterator012<std::bidirectional_iterator_tag,CLeaderboardPage<CAsynchronousBragEntry<__int64>,__int64>,__int64,CLeaderboardPage<CAsynchronousBragEntry<__int64>,__int64> const *,CLeaderboardPage<CAsynchronousBragEntry<__int64>,__int64> const &,std::_Iterator_base0>"
        ]
    );
}

#[test]
fn test_type_545() {
    let result = parse_type(
        "struct __cppobj std::_Iterator012<std::bidirectional_iterator_tag,IPfxInstance const *,__int64,IPfxInstance const * const *,IPfxInstance const * const &,std::_Iterator_base0> : std::_Iterator_base0",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "_Iterator012<std::bidirectional_iterator_tag,IPfxInstance const *,__int64,IPfxInstance const * const *,IPfxInstance const * const &,std::_Iterator_base0>"
        ]
    );
}

#[test]
fn test_type_546() {
    let result = parse_type(
        "struct __cppobj std::_Iterator012<std::bidirectional_iterator_tag,NVehicle_Hidden::SVehicleMessage,__int64,NVehicle_Hidden::SVehicleMessage const *,NVehicle_Hidden::SVehicleMessage const &,std::_Iterator_base0> : std::_Iterator_base0",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "_Iterator012<std::bidirectional_iterator_tag,NVehicle_Hidden::SVehicleMessage,__int64,NVehicle_Hidden::SVehicleMessage const *,NVehicle_Hidden::SVehicleMessage const &,std::_Iterator_base0>"
        ]
    );
}

#[test]
fn test_type_547() {
    let result = parse_type(
        "struct __cppobj std::_Iterator012<std::random_access_iterator_tag,boost::shared_ptr<CSoundResource>,__int64,boost::shared_ptr<CSoundResource> const *,boost::shared_ptr<CSoundResource> const &,std::_Iterator_base0> : std::_Iterator_base0",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "_Iterator012<std::random_access_iterator_tag,boost::shared_ptr<CSoundResource>,__int64,boost::shared_ptr<CSoundResource> const *,boost::shared_ptr<CSoundResource> const &,std::_Iterator_base0>"
        ]
    );
}

#[test]
fn test_type_548() {
    let result = parse_type(
        "struct __cppobj std::_Iterator012<std::random_access_iterator_tag,CMessageboxManager::SMsgboxButtonPressInfo,__int64,CMessageboxManager::SMsgboxButtonPressInfo const *,CMessageboxManager::SMsgboxButtonPressInfo const &,std::_Iterator_base0> : std::_Iterator_base0",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "_Iterator012<std::random_access_iterator_tag,CMessageboxManager::SMsgboxButtonPressInfo,__int64,CMessageboxManager::SMsgboxButtonPressInfo const *,CMessageboxManager::SMsgboxButtonPressInfo const &,std::_Iterator_base0>"
        ]
    );
}

#[test]
fn test_type_549() {
    let result = parse_type(
        "struct __cppobj std::_Iterator012<std::random_access_iterator_tag,CVehicle::SProcedurallyAnimatedPartInfo,__int64,CVehicle::SProcedurallyAnimatedPartInfo const *,CVehicle::SProcedurallyAnimatedPartInfo const &,std::_Iterator_base0> : std::_Iterator_base0",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "_Iterator012<std::random_access_iterator_tag,CVehicle::SProcedurallyAnimatedPartInfo,__int64,CVehicle::SProcedurallyAnimatedPartInfo const *,CVehicle::SProcedurallyAnimatedPartInfo const &,std::_Iterator_base0>"
        ]
    );
}

#[test]
fn test_type_550() {
    let result = parse_type(
        "struct __cppobj std::_Iterator012<std::random_access_iterator_tag,NAttachedEffectContainer::SDynamicAttachedEffectBase *,__int64,NAttachedEffectContainer::SDynamicAttachedEffectBase * const *,NAttachedEffectContainer::SDynamicAttachedEffectBase * const &,std::_Iterator_base0> : std::_Iterator_base0",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "_Iterator012<std::random_access_iterator_tag,NAttachedEffectContainer::SDynamicAttachedEffectBase *,__int64,NAttachedEffectContainer::SDynamicAttachedEffectBase * const *,NAttachedEffectContainer::SDynamicAttachedEffectBase * const &,std::_Iterator_base0>"
        ]
    );
}

#[test]
fn test_type_551() {
    let result = parse_type(
        "struct __cppobj std::_Iterator012<std::random_access_iterator_tag,NEnvironmentPreset::SEnvironmentPresetRender,__int64,NEnvironmentPreset::SEnvironmentPresetRender const *,NEnvironmentPreset::SEnvironmentPresetRender const &,std::_Iterator_base0> : std::_Iterator_base0",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "_Iterator012<std::random_access_iterator_tag,NEnvironmentPreset::SEnvironmentPresetRender,__int64,NEnvironmentPreset::SEnvironmentPresetRender const *,NEnvironmentPreset::SEnvironmentPresetRender const &,std::_Iterator_base0>"
        ]
    );
}

#[test]
fn test_type_552() {
    let result = parse_type(
        "struct __cppobj std::_Iterator012<std::random_access_iterator_tag,std::pair<boost::weak_ptr<CAsynchronousBragThresholdObserver<float> >,float>,__int64,std::pair<boost::weak_ptr<CAsynchronousBragThresholdObserver<float> >,float> const *,std::pair<boost::weak_ptr<CAsynchronousBragThresholdObserver<float> >,float> const &,std::_Iterator_base0> : std::_Iterator_base0",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "_Iterator012<std::random_access_iterator_tag,std::pair<boost::weak_ptr<CAsynchronousBragThresholdObserver<float> >,float>,__int64,std::pair<boost::weak_ptr<CAsynchronousBragThresholdObserver<float> >,float> const *,std::pair<boost::weak_ptr<CAsynchronousBragThresholdObserver<float> >,float> const &,std::_Iterator_base0>"
        ]
    );
}

#[test]
fn test_type_553() {
    let result = parse_type(
        "struct __cppobj std::_Iterator012<std::random_access_iterator_tag,std::pair<CHashString,bool>,__int64,std::pair<CHashString,bool> const *,std::pair<CHashString,bool> const &,std::_Iterator_base0> : std::_Iterator_base0",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "_Iterator012<std::random_access_iterator_tag,std::pair<CHashString,bool>,__int64,std::pair<CHashString,bool> const *,std::pair<CHashString,bool> const &,std::_Iterator_base0>"
        ]
    );
}

#[test]
fn test_type_554() {
    let result = parse_type(
        "struct __cppobj std::_List_iterator<std::_List_val<CHUDUI::SPOIRecord> > : std::_List_const_iterator<std::_List_val<CHUDUI::SPOIRecord> >",
    );
    assert_eq!(
        result,
        vec!["std", "_List_iterator<std::_List_val<CHUDUI::SPOIRecord> >"]
    );
}

#[test]
fn test_type_555() {
    let result = parse_type(
        "struct __cppobj std::_List_unchecked_const_iterator<std::_List_val<CSpawnSystem::SSpawnRequest>,std::_Iterator_base0> : std::_Iterator012<std::bidirectional_iterator_tag,CSpawnSystem::SSpawnRequest,__int64,CSpawnSystem::SSpawnRequest const *,CSpawnSystem::SSpawnRequest const &,std::_Iterator_base0>",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "_List_unchecked_const_iterator<std::_List_val<CSpawnSystem::SSpawnRequest>,std::_Iterator_base0>"
        ]
    );
}

#[test]
fn test_type_556() {
    let result = parse_type(
        "struct __cppobj std::_List_val<CEncounterManager::SEncounterCooldown> : std::_List_nod<CEncounterManager::SEncounterCooldown>",
    );
    assert_eq!(
        result,
        vec!["std", "_List_val<CEncounterManager::SEncounterCooldown>"]
    );
}

#[test]
fn test_type_557() {
    let result = parse_type(
        "struct __cppobj std::_Pair_base<boost::container::container_detail::vec_iterator<boost::container::container_detail::pair<CHashString,CStatisticDelegateHolder<TDelegate<void __cdecl(CStatistic<__int64> const *,__int64,__int64,CStatisticContext const &)> > > *,0>,boost::container::container_detail::vec_iterator<boost::container::container_detail::pair<CHashString,CStatisticDelegateHolder<TDelegate<void __cdecl(CStatistic<__int64> const *,__int64,__int64,CStatisticContext const &)> > > *,0> >",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "_Pair_base<boost::container::container_detail::vec_iterator<boost::container::container_detail::pair<CHashString,CStatisticDelegateHolder<TDelegate<void __cdecl(CStatistic<__int64> const *,__int64,__int64,CStatisticContext const &)> > > *,0>,boost::container::container_detail::vec_iterator<boost::container::container_detail::pair<CHashString,CStatisticDelegateHolder<TDelegate<void __cdecl(CStatistic<__int64> const *,__int64,__int64,CStatisticContext const &)> > > *,0> >"
        ]
    );
}

#[test]
fn test_type_558() {
    let result = parse_type("struct __cppobj std::_Pair_base<CHashString,STaskGroupInfo>");
    assert_eq!(
        result,
        vec!["std", "_Pair_base<CHashString,STaskGroupInfo>"]
    );
}

#[test]
fn test_type_559() {
    let result = parse_type(
        "struct __cppobj std::_Pair_base<CHeatRule::SSpawnSeat *,CHeatRule::SSpawnSeat *>",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "_Pair_base<CHeatRule::SSpawnSeat *,CHeatRule::SSpawnSeat *>"
        ]
    );
}

#[test]
fn test_type_560() {
    let result = parse_type("struct __cppobj std::_Pair_base<CRuntimeContainer,CRigidObject *>");
    assert_eq!(
        result,
        vec!["std", "_Pair_base<CRuntimeContainer,CRigidObject *>"]
    );
}

#[test]
fn test_type_561() {
    let result = parse_type("struct __cppobj std::_Pair_base<short,unsigned short>");
    assert_eq!(result, vec!["std", "_Pair_base<short,unsigned short>"]);
}

#[test]
fn test_type_562() {
    let result = parse_type(
        "struct __cppobj std::_Pair_base<std::_Tree_iterator<std::_Tree_val<std::_Tmap_traits<unsigned int,NAnimationSystem::SConditionConstructInfo,std::less<unsigned int>,std::allocator<std::pair<unsigned int const ,NAnimationSystem::SConditionConstructInfo> >,0> > >,std::_Tree_iterator<std::_Tree_val<std::_Tmap_traits<unsigned int,NAnimationSystem::SConditionConstructInfo,std::less<unsigned int>,std::allocator<std::pair<unsigned int const ,NAnimationSystem::SConditionConstructInfo> >,0> > > >",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "_Pair_base<std::_Tree_iterator<std::_Tree_val<std::_Tmap_traits<unsigned int,NAnimationSystem::SConditionConstructInfo,std::less<unsigned int>,std::allocator<std::pair<unsigned int const ,NAnimationSystem::SConditionConstructInfo> >,0> > >,std::_Tree_iterator<std::_Tree_val<std::_Tmap_traits<unsigned int,NAnimationSystem::SConditionConstructInfo,std::less<unsigned int>,std::allocator<std::pair<unsigned int const ,NAnimationSystem::SConditionConstructInfo> >,0> > > >"
        ]
    );
}

#[test]
fn test_type_563() {
    let result =
        parse_type("struct __cppobj std::_Pair_base<unsigned int const ,SPatchCacheEntry>");
    assert_eq!(
        result,
        vec!["std", "_Pair_base<unsigned int const ,SPatchCacheEntry>"]
    );
}

#[test]
fn test_type_564() {
    let result = parse_type(
        "struct __cppobj std::_Revranit<std::_Vector_iterator<std::_Vector_val<unsigned short> >,std::iterator<std::random_access_iterator_tag,unsigned short,__int64,unsigned short *,unsigned short &> > : std::iterator<std::random_access_iterator_tag,unsigned short,__int64,unsigned short *,unsigned short &>",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "_Revranit<std::_Vector_iterator<std::_Vector_val<unsigned short> >,std::iterator<std::random_access_iterator_tag,unsigned short,__int64,unsigned short *,unsigned short &> >"
        ]
    );
}

#[test]
fn test_type_565() {
    let result = parse_type(
        "struct __cppobj std::_Tree_const_iterator<std::_Tree_val<std::_Tmap_traits<std::string,std::vector<CColorRGBAb>,std::less<std::string >,std::allocator<std::pair<std::string const ,std::vector<CColorRGBAb> > >,0> > > : std::_Tree_unchecked_const_iterator<std::_Tree_val<std::_Tmap_traits<std::string,std::vector<CColorRGBAb>,std::less<std::string >,std::allocator<std::pair<std::string const ,std::vector<CColorRGBAb> > >,0> >,std::_Iterator_base0>",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "_Tree_const_iterator<std::_Tree_val<std::_Tmap_traits<std::string,std::vector<CColorRGBAb>,std::less<std::string >,std::allocator<std::pair<std::string const ,std::vector<CColorRGBAb> > >,0> > >"
        ]
    );
}

#[test]
fn test_type_566() {
    let result = parse_type(
        "struct __cppobj std::_Tree_unchecked_const_iterator<std::_Tree_val<std::_Tmap_traits<bool *,std::set<bool *>,std::less<bool *>,std::allocator<std::pair<bool * const,std::set<bool *> > >,0> >,std::_Iterator_base0> : std::_Iterator012<std::bidirectional_iterator_tag,std::pair<bool * const,std::set<bool *> >,__int64,std::pair<bool * const,std::set<bool *> > const *,std::pair<bool * const,std::set<bool *> > const &,std::_Iterator_base0>",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "_Tree_unchecked_const_iterator<std::_Tree_val<std::_Tmap_traits<bool *,std::set<bool *>,std::less<bool *>,std::allocator<std::pair<bool * const,std::set<bool *> > >,0> >,std::_Iterator_base0>"
        ]
    );
}

#[test]
fn test_type_567() {
    let result = parse_type(
        "struct __cppobj std::_Tree_val<std::_Tmap_traits<std::string,NFMODEventInfo::SFMODEventInfo,std::less<std::string >,std::allocator<std::pair<std::string const ,NFMODEventInfo::SFMODEventInfo> >,0> > : std::_Tree_nod<std::_Tmap_traits<std::string,NFMODEventInfo::SFMODEventInfo,std::less<std::string >,std::allocator<std::pair<std::string const ,NFMODEventInfo::SFMODEventInfo> >,0> >",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "_Tree_val<std::_Tmap_traits<std::string,NFMODEventInfo::SFMODEventInfo,std::less<std::string >,std::allocator<std::pair<std::string const ,NFMODEventInfo::SFMODEventInfo> >,0> >"
        ]
    );
}

#[test]
fn test_type_568() {
    let result = parse_type(
        "struct __cppobj std::_Tree<std::_Tset_traits<char const *,std::less<char const *>,std::allocator<char const *>,0> > : std::_Tree_val<std::_Tset_traits<char const *,std::less<char const *>,std::allocator<char const *>,0> >",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "_Tree<std::_Tset_traits<char const *,std::less<char const *>,std::allocator<char const *>,0> >"
        ]
    );
}

#[test]
fn test_type_569() {
    let result = parse_type(
        "struct __cppobj std::_Unique_ptr_base<CLocation,std::default_delete<CLocation>,1> : std::default_delete<CLocation>",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "_Unique_ptr_base<CLocation,std::default_delete<CLocation>,1>"
        ]
    );
}

#[test]
fn test_type_570() {
    let result = parse_type(
        "struct __cppobj std::_Vector_const_iterator<std::_Vector_val<boost::shared_ptr<CAnimationInstance>> > : std::_Iterator012<std::random_access_iterator_tag,boost::shared_ptr<CAnimationInstance>,__int64,boost::shared_ptr<CAnimationInstance> const *,boost::shared_ptr<CAnimationInstance> const &,std::_Iterator_base0>",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "_Vector_const_iterator<std::_Vector_val<boost::shared_ptr<CAnimationInstance>> >"
        ]
    );
}

#[test]
fn test_type_571() {
    let result = parse_type(
        "struct __cppobj std::_Vector_const_iterator<std::_Vector_val<CLocation *> > : std::_Iterator012<std::random_access_iterator_tag,CLocation *,__int64,CLocation * const *,CLocation * const &,std::_Iterator_base0>",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "_Vector_const_iterator<std::_Vector_val<CLocation *> >"
        ]
    );
}

#[test]
fn test_type_572() {
    let result = parse_type(
        "struct __cppobj std::_Vector_const_iterator<std::_Vector_val<CRoadDecorationCreator::SSpawnInfo> > : std::_Iterator012<std::random_access_iterator_tag,CRoadDecorationCreator::SSpawnInfo,__int64,CRoadDecorationCreator::SSpawnInfo const *,CRoadDecorationCreator::SSpawnInfo const &,std::_Iterator_base0>",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "_Vector_const_iterator<std::_Vector_val<CRoadDecorationCreator::SSpawnInfo> >"
        ]
    );
}

#[test]
fn test_type_573() {
    let result = parse_type(
        "struct __cppobj std::_Vector_const_iterator<std::_Vector_val<NAnimationSystem::CHumanIK::sTQS> > : std::_Iterator012<std::random_access_iterator_tag,NAnimationSystem::CHumanIK::sTQS,__int64,NAnimationSystem::CHumanIK::sTQS const *,NAnimationSystem::CHumanIK::sTQS const &,std::_Iterator_base0>",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "_Vector_const_iterator<std::_Vector_val<NAnimationSystem::CHumanIK::sTQS> >"
        ]
    );
}

#[test]
fn test_type_574() {
    let result = parse_type(
        "struct __cppobj std::_Vector_const_iterator<std::_Vector_val<std::pair<CMetaAnimateUV *,NGraphicsEngine::SAnimatedUVStrategy>> > : std::_Iterator012<std::random_access_iterator_tag,std::pair<CMetaAnimateUV *,NGraphicsEngine::SAnimatedUVStrategy>,__int64,std::pair<CMetaAnimateUV *,NGraphicsEngine::SAnimatedUVStrategy> const *,std::pair<CMetaAnimateUV *,NGraphicsEngine::SAnimatedUVStrategy> const &,std::_Iterator_base0>",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "_Vector_const_iterator<std::_Vector_val<std::pair<CMetaAnimateUV *,NGraphicsEngine::SAnimatedUVStrategy>> >"
        ]
    );
}

#[test]
fn test_type_575() {
    let result = parse_type(
        "struct __cppobj std::_Vector_const_iterator<std::_Vector_val<STrafficLight *> > : std::_Iterator012<std::random_access_iterator_tag,STrafficLight *,__int64,STrafficLight * const *,STrafficLight * const &,std::_Iterator_base0>",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "_Vector_const_iterator<std::_Vector_val<STrafficLight *> >"
        ]
    );
}

#[test]
fn test_type_576() {
    let result = parse_type(
        "struct __cppobj std::_Vector_const_iterator<std::_Vector_val<void *> > : std::_Iterator012<std::random_access_iterator_tag,void *,__int64,void * const *,void * const &,std::_Iterator_base0>",
    );
    assert_eq!(
        result,
        vec!["std", "_Vector_const_iterator<std::_Vector_val<void *> >"]
    );
}

#[test]
fn test_type_577() {
    let result = parse_type(
        "struct __cppobj std::_Vector_iterator<std::_Vector_val<boost::weak_ptr<CFriendObserver>> > : std::_Vector_const_iterator<std::_Vector_val<boost::weak_ptr<CFriendObserver>> >",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "_Vector_iterator<std::_Vector_val<boost::weak_ptr<CFriendObserver>> >"
        ]
    );
}

#[test]
fn test_type_578() {
    let result = parse_type(
        "struct __cppobj std::_Vector_iterator<std::_Vector_val<CAchievementsHandler<CFeatsManager,CFeatLink>::CStatAchievementLink<float> *,std::allocator<CAchievementsHandler<CFeatsManager,CFeatLink>::CStatAchievementLink<float> *> > > : std::_Vector_const_iterator<std::_Vector_val<CAchievementsHandler<CFeatsManager,CFeatLink>::CStatAchievementLink<float> *,std::allocator<CAchievementsHandler<CFeatsManager,CFeatLink>::CStatAchievementLink<float> *> > >",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "_Vector_iterator<std::_Vector_val<CAchievementsHandler<CFeatsManager,CFeatLink>::CStatAchievementLink<float> *,std::allocator<CAchievementsHandler<CFeatsManager,CFeatLink>::CStatAchievementLink<float> *> > >"
        ]
    );
}

#[test]
fn test_type_579() {
    let result = parse_type(
        "struct __cppobj std::_Vector_iterator<std::_Vector_val<CAIUtilityMap::SMeshFace> > : std::_Vector_const_iterator<std::_Vector_val<CAIUtilityMap::SMeshFace> >",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "_Vector_iterator<std::_Vector_val<CAIUtilityMap::SMeshFace> >"
        ]
    );
}

#[test]
fn test_type_580() {
    let result = parse_type(
        "struct __cppobj std::_Vector_iterator<std::_Vector_val<CMedianKeeper::SQueueElement> > : std::_Vector_const_iterator<std::_Vector_val<CMedianKeeper::SQueueElement> >",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "_Vector_iterator<std::_Vector_val<CMedianKeeper::SQueueElement> >"
        ]
    );
}

#[test]
fn test_type_581() {
    let result = parse_type(
        "struct __cppobj std::_Vector_iterator<std::_Vector_val<CSequenceObject2::SSeqTransformTrack> > : std::_Vector_const_iterator<std::_Vector_val<CSequenceObject2::SSeqTransformTrack> >",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "_Vector_iterator<std::_Vector_val<CSequenceObject2::SSeqTransformTrack> >"
        ]
    );
}

#[test]
fn test_type_582() {
    let result = parse_type(
        "struct __cppobj std::_Vector_iterator<std::_Vector_val<PlayerId_Steam> > : std::_Vector_const_iterator<std::_Vector_val<PlayerId_Steam> >",
    );
    assert_eq!(
        result,
        vec!["std", "_Vector_iterator<std::_Vector_val<PlayerId_Steam> >"]
    );
}

#[test]
fn test_type_583() {
    let result = parse_type(
        "struct __cppobj std::_Vector_iterator<std::_Vector_val<std::pair<CMetaAnimateUV *,NGraphicsEngine::SAnimatedUVStrategy>> > : std::_Vector_const_iterator<std::_Vector_val<std::pair<CMetaAnimateUV *,NGraphicsEngine::SAnimatedUVStrategy>> >",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "_Vector_iterator<std::_Vector_val<std::pair<CMetaAnimateUV *,NGraphicsEngine::SAnimatedUVStrategy>> >"
        ]
    );
}

#[test]
fn test_type_584() {
    let result = parse_type(
        "struct __cppobj std::_Vector_iterator<std::_Vector_val<unsigned __int64> > : std::_Vector_const_iterator<std::_Vector_val<unsigned __int64> >",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "_Vector_iterator<std::_Vector_val<unsigned __int64> >"
        ]
    );
}

#[test]
fn test_type_585() {
    let result = parse_type(
        "struct __cppobj std::allocator<boost::shared_ptr<SFMODBankInfo> > : std::_Allocator_base<boost::shared_ptr<SFMODBankInfo> >",
    );
    assert_eq!(
        result,
        vec!["std", "allocator<boost::shared_ptr<SFMODBankInfo> >"]
    );
}

#[test]
fn test_type_586() {
    let result = parse_type(
        "struct __cppobj std::allocator<CAITrafficManager::SRoadGraphFilter>::rebind<CAITrafficManager::SRoadGraphFilter>",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "allocator<CAITrafficManager::SRoadGraphFilter>",
            "rebind<CAITrafficManager::SRoadGraphFilter>"
        ]
    );
}

#[test]
fn test_type_587() {
    let result = parse_type(
        "struct __cppobj std::allocator<CBatchRegisterEvents::UMemoryBlock> : std::_Allocator_base<CBatchRegisterEvents::UMemoryBlock>",
    );
    assert_eq!(
        result,
        vec!["std", "allocator<CBatchRegisterEvents::UMemoryBlock>"]
    );
}

#[test]
fn test_type_588() {
    let result = parse_type(
        "struct __cppobj std::allocator<CEventTrigger::SEventTriggerComponent>::rebind<CEventTrigger::SEventTriggerComponent>",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "allocator<CEventTrigger::SEventTriggerComponent>",
            "rebind<CEventTrigger::SEventTriggerComponent>"
        ]
    );
}

#[test]
fn test_type_589() {
    let result = parse_type(
        "struct __cppobj std::allocator<CHeatManager::SHeatRoad> : std::_Allocator_base<CHeatManager::SHeatRoad>",
    );
    assert_eq!(result, vec!["std", "allocator<CHeatManager::SHeatRoad>"]);
}

#[test]
fn test_type_590() {
    let result = parse_type(
        "struct __cppobj std::allocator<CHUDUI::SWireData> : std::_Allocator_base<CHUDUI::SWireData>",
    );
    assert_eq!(result, vec!["std", "allocator<CHUDUI::SWireData>"]);
}

#[test]
fn test_type_591() {
    let result = parse_type(
        "struct __cppobj std::allocator<CPlayerReportingEvent::TEventNode>::rebind<CPlayerReportingEvent::TEventNode *>",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "allocator<CPlayerReportingEvent::TEventNode>",
            "rebind<CPlayerReportingEvent::TEventNode *>"
        ]
    );
}

#[test]
fn test_type_592() {
    let result = parse_type(
        "struct __cppobj std::allocator<CProvince::SRebelSwap>::rebind<CProvince::SRebelSwap>",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "allocator<CProvince::SRebelSwap>",
            "rebind<CProvince::SRebelSwap>"
        ]
    );
}

#[test]
fn test_type_593() {
    let result = parse_type(
        "struct __cppobj std::allocator<CSequenceObject2::SReferencedObject>::rebind<CSequenceObject2::SReferencedObject>",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "allocator<CSequenceObject2::SReferencedObject>",
            "rebind<CSequenceObject2::SReferencedObject>"
        ]
    );
}

#[test]
fn test_type_594() {
    let result =
        parse_type("struct __cppobj std::allocator<CUIBase *> : std::_Allocator_base<CUIBase *>");
    assert_eq!(result, vec!["std", "allocator<CUIBase *>"]);
}

#[test]
fn test_type_595() {
    let result = parse_type(
        "struct __cppobj std::allocator<IStatisticProvider::SEntry> : std::_Allocator_base<IStatisticProvider::SEntry>",
    );
    assert_eq!(result, vec!["std", "allocator<IStatisticProvider::SEntry>"]);
}

#[test]
fn test_type_596() {
    let result = parse_type(
        "struct __cppobj std::allocator<NRevolution::SResupplyPointRecord>::rebind<NRevolution::SResupplyPointRecord>",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "allocator<NRevolution::SResupplyPointRecord>",
            "rebind<NRevolution::SResupplyPointRecord>"
        ]
    );
}

#[test]
fn test_type_597() {
    let result = parse_type(
        "struct __cppobj std::allocator<NSaveLoad::Test::NonDefaultContructableUserType> : std::_Allocator_base<NSaveLoad::Test::NonDefaultContructableUserType>",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "allocator<NSaveLoad::Test::NonDefaultContructableUserType>"
        ]
    );
}

#[test]
fn test_type_598() {
    let result = parse_type(
        "struct __cppobj std::allocator<STaskGroupInfo const >::rebind<STaskGroupInfo const >",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "allocator<STaskGroupInfo const >",
            "rebind<STaskGroupInfo const >"
        ]
    );
}

#[test]
fn test_type_599() {
    let result = parse_type(
        "struct __cppobj std::allocator<STaskInfo const *>::rebind<std::_Container_proxy>",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "allocator<STaskInfo const *>",
            "rebind<std::_Container_proxy>"
        ]
    );
}

#[test]
fn test_type_600() {
    let result = parse_type(
        "struct __cppobj std::allocator<std::_Tree_nod<std::_Tmap_traits<int,std::string,std::less<int>,std::allocator<std::pair<int const ,std::string > >,0> >::_Node> : std::_Allocator_base<std::_Tree_nod<std::_Tmap_traits<int,std::string,std::less<int>,std::allocator<std::pair<int const ,std::string > >,0> >::_Node>",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "allocator<std::_Tree_nod<std::_Tmap_traits<int,std::string,std::less<int>,std::allocator<std::pair<int const ,std::string > >,0> >::_Node>"
        ]
    );
}

#[test]
fn test_type_601() {
    let result = parse_type(
        "struct __cppobj std::allocator<std::pair<CHashString const ,CAbilitiesHandler::SAbilityStatus> >::rebind<std::_List_nod<std::pair<CHashString const ,CAbilitiesHandler::SAbilityStatus>>::_Node>",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "allocator<std::pair<CHashString const ,CAbilitiesHandler::SAbilityStatus> >",
            "rebind<std::_List_nod<std::pair<CHashString const ,CAbilitiesHandler::SAbilityStatus>>::_Node>"
        ]
    );
}

#[test]
fn test_type_602() {
    let result = parse_type(
        "struct __cppobj std::allocator<std::pair<hknpBodyId const ,CPfxDebugDisplayListener::SGeometryDisplayInfo> >::rebind<std::_Tree_nod<std::_Tmap_traits<hknpBodyId,CPfxDebugDisplayListener::SGeometryDisplayInfo,std::less<hknpBodyId>,std::allocator<std::pair<hknpBodyId const ,CPfxDebugDisplayListener::SGeometryDisplayInfo> >,0> >::_Node>",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "allocator<std::pair<hknpBodyId const ,CPfxDebugDisplayListener::SGeometryDisplayInfo> >",
            "rebind<std::_Tree_nod<std::_Tmap_traits<hknpBodyId,CPfxDebugDisplayListener::SGeometryDisplayInfo,std::less<hknpBodyId>,std::allocator<std::pair<hknpBodyId const ,CPfxDebugDisplayListener::SGeometryDisplayInfo> >,0> >::_Node>"
        ]
    );
}

#[test]
fn test_type_603() {
    let result = parse_type(
        "struct __cppobj std::allocator<std::pair<hknpBodyId const ,int> >::rebind<std::_Tree_nod<std::_Tmap_traits<hknpBodyId,int,std::less<hknpBodyId>,std::allocator<std::pair<hknpBodyId const ,int> >,0> >::_Node>",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "allocator<std::pair<hknpBodyId const ,int> >",
            "rebind<std::_Tree_nod<std::_Tmap_traits<hknpBodyId,int,std::less<hknpBodyId>,std::allocator<std::pair<hknpBodyId const ,int> >,0> >::_Node>"
        ]
    );
}

#[test]
fn test_type_604() {
    let result = parse_type(
        "struct __cppobj std::allocator<std::pair<hknpBodyId const ,unsigned int> > : std::_Allocator_base<std::pair<hknpBodyId const ,unsigned int> >",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "allocator<std::pair<hknpBodyId const ,unsigned int> >"
        ]
    );
}

#[test]
fn test_type_605() {
    let result = parse_type(
        "struct __cppobj std::allocator<std::pair<int const ,CSteeringUtil::SDevKey> > : std::_Allocator_base<std::pair<int const ,CSteeringUtil::SDevKey> >",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "allocator<std::pair<int const ,CSteeringUtil::SDevKey> >"
        ]
    );
}

#[test]
fn test_type_606() {
    let result = parse_type(
        "struct __cppobj std::allocator<std::pair<std::string const ,boost::function<float __cdecl(void)> > > : std::_Allocator_base<std::pair<std::string const ,boost::function<float __cdecl(void)> > >",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "allocator<std::pair<std::string const ,boost::function<float __cdecl(void)> > >"
        ]
    );
}

#[test]
fn test_type_607() {
    let result = parse_type(
        "struct __cppobj std::allocator<std::pair<unsigned __int64 const ,NMissionSystem::SVehicleSaveSlot> >::rebind<std::_List_iterator<std::_List_val<std::pair<unsigned __int64 const ,NMissionSystem::SVehicleSaveSlot>> > >",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "allocator<std::pair<unsigned __int64 const ,NMissionSystem::SVehicleSaveSlot> >",
            "rebind<std::_List_iterator<std::_List_val<std::pair<unsigned __int64 const ,NMissionSystem::SVehicleSaveSlot>> > >"
        ]
    );
}

#[test]
fn test_type_608() {
    let result = parse_type(
        "struct __cppobj std::allocator<std::pair<unsigned int const ,int> > : std::_Allocator_base<std::pair<unsigned int const ,int> >",
    );
    assert_eq!(
        result,
        vec!["std", "allocator<std::pair<unsigned int const ,int> >"]
    );
}

#[test]
fn test_type_609() {
    let result = parse_type(
        "struct __cppobj std::allocator<std::pair<unsigned short,std::vector<CAmbientSpawnPoint *> > >::rebind<std::pair<unsigned short,std::vector<CAmbientSpawnPoint *> > >",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "allocator<std::pair<unsigned short,std::vector<CAmbientSpawnPoint *> > >",
            "rebind<std::pair<unsigned short,std::vector<CAmbientSpawnPoint *> > >"
        ]
    );
}

#[test]
fn test_type_610() {
    let result = parse_type(
        "struct __cppobj std::allocator<std::tr1::_Function_impl0<void> >::rebind<std::tr1::_Impl_no_alloc0<std::tr1::_Callable_obj<NPfxBreakable::SDeferredAngularImpulseAtPointApplicator_Local<CPfxBreakable,3773750015>,0>,void> >",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "allocator<std::tr1::_Function_impl0<void> >",
            "rebind<std::tr1::_Impl_no_alloc0<std::tr1::_Callable_obj<NPfxBreakable::SDeferredAngularImpulseAtPointApplicator_Local<CPfxBreakable,3773750015>,0>,void> >"
        ]
    );
}

#[test]
fn test_type_611() {
    let result = parse_type(
        "struct __cppobj std::allocator<std::tr1::_Function_impl0<void> >::rebind<std::tr1::_Impl_no_alloc0<std::tr1::_Callable_obj<NPfxBreakable::SDeferredAngularImpulseAtPointApplicator_World<CPfxBreakable,1085760493>,0>,void> >",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "allocator<std::tr1::_Function_impl0<void> >",
            "rebind<std::tr1::_Impl_no_alloc0<std::tr1::_Callable_obj<NPfxBreakable::SDeferredAngularImpulseAtPointApplicator_World<CPfxBreakable,1085760493>,0>,void> >"
        ]
    );
}

#[test]
fn test_type_612() {
    let result = parse_type(
        "struct __cppobj std::allocator<std::tr1::_Function_impl1<bool,CFriend const &> >::rebind<std::tr1::_Impl_no_alloc1<std::tr1::_Callable_obj<<lambda102>,0>,bool,CFriend const &> >",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "allocator<std::tr1::_Function_impl1<bool,CFriend const &> >",
            "rebind<std::tr1::_Impl_no_alloc1<std::tr1::_Callable_obj<<lambda102>,0>,bool,CFriend const &> >"
        ]
    );
}

#[test]
fn test_type_613() {
    let result = parse_type(
        "struct __cppobj std::allocator<std::tr1::_Function_impl1<void,CCallContext &> >::rebind<std::tr1::_Impl_no_alloc1<std::tr1::_Callable_obj<<lambda54>,0>,void,CCallContext &> >",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "allocator<std::tr1::_Function_impl1<void,CCallContext &> >",
            "rebind<std::tr1::_Impl_no_alloc1<std::tr1::_Callable_obj<<lambda54>,0>,void,CCallContext &> >"
        ]
    );
}

#[test]
fn test_type_614() {
    let result = parse_type(
        "struct __cppobj std::allocator<std::tr1::_Function_impl1<void,COnlineQueryResult const &> >::rebind<std::tr1::_Impl_no_alloc1<std::tr1::_Callable_obj<<lambda61>,0>,void,COnlineQueryResult const &> >",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "allocator<std::tr1::_Function_impl1<void,COnlineQueryResult const &> >",
            "rebind<std::tr1::_Impl_no_alloc1<std::tr1::_Callable_obj<<lambda61>,0>,void,COnlineQueryResult const &> >"
        ]
    );
}

#[test]
fn test_type_615() {
    let result = parse_type(
        "struct __cppobj std::allocator<std::tr1::_Function_impl1<void,COnlineQueryResult const &> >::rebind<std::tr1::_Impl_no_alloc1<std::tr1::_Callable_obj<<lambda63>,0>,void,COnlineQueryResult const &> >",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "allocator<std::tr1::_Function_impl1<void,COnlineQueryResult const &> >",
            "rebind<std::tr1::_Impl_no_alloc1<std::tr1::_Callable_obj<<lambda63>,0>,void,COnlineQueryResult const &> >"
        ]
    );
}

#[test]
fn test_type_616() {
    let result = parse_type(
        "struct __cppobj std::allocator<std::tr1::_Function_impl1<void,enum NOnline::EErrorCode> >::rebind<std::tr1::_Impl_no_alloc1<std::tr1::_Callable_obj<<lambda140>,0>,void,enum NOnline::EErrorCode> >",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "allocator<std::tr1::_Function_impl1<void,enum NOnline::EErrorCode> >",
            "rebind<std::tr1::_Impl_no_alloc1<std::tr1::_Callable_obj<<lambda140>,0>,void,enum NOnline::EErrorCode> >"
        ]
    );
}

#[test]
fn test_type_617() {
    let result = parse_type(
        "struct __cppobj std::allocator<std::tr1::_Function_impl2<void,CLeaderboardPage<CAsynchronousBragEntry<float>,float> &,enum NOnline::EErrorCode> >::rebind<std::tr1::_Impl_no_alloc2<std::tr1::_Callable_obj<<lambda54>,0>,void,CLeaderboardPage<CAsynchronousBragEntry<float>,float> &,enum NOnline::EErrorCode> >",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "allocator<std::tr1::_Function_impl2<void,CLeaderboardPage<CAsynchronousBragEntry<float>,float> &,enum NOnline::EErrorCode> >",
            "rebind<std::tr1::_Impl_no_alloc2<std::tr1::_Callable_obj<<lambda54>,0>,void,CLeaderboardPage<CAsynchronousBragEntry<float>,float> &,enum NOnline::EErrorCode> >"
        ]
    );
}

#[test]
fn test_type_618() {
    let result = parse_type(
        "struct __cppobj std::allocator<std::tr1::_Impl_no_alloc0<std::tr1::_Callable_obj<<lambda8>,0>,void> > : std::_Allocator_base<std::tr1::_Impl_no_alloc0<std::tr1::_Callable_obj<<lambda8>,0>,void> >",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "allocator<std::tr1::_Impl_no_alloc0<std::tr1::_Callable_obj<<lambda8>,0>,void> >"
        ]
    );
}

#[test]
fn test_type_619() {
    let result = parse_type(
        "struct __cppobj std::allocator<std::tr1::_Impl_no_alloc0<std::tr1::_Callable_obj<NPfxBreakable::SDeferredFrictionAngularImpulseApplicator_World<CPfxBreakable,3585189063>,0>,void> > : std::_Allocator_base<std::tr1::_Impl_no_alloc0<std::tr1::_Callable_obj<NPfxBreakable::SDeferredFrictionAngularImpulseApplicator_World<CPfxBreakable,3585189063>,0>,void> >",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "allocator<std::tr1::_Impl_no_alloc0<std::tr1::_Callable_obj<NPfxBreakable::SDeferredFrictionAngularImpulseApplicator_World<CPfxBreakable,3585189063>,0>,void> >"
        ]
    );
}

#[test]
fn test_type_620() {
    let result = parse_type(
        "struct __cppobj std::allocator<std::tr1::_Impl_no_alloc1<std::tr1::_Callable_obj<<lambda162>,0>,void,COnlineQueryResult const &> > : std::_Allocator_base<std::tr1::_Impl_no_alloc1<std::tr1::_Callable_obj<<lambda162>,0>,void,COnlineQueryResult const &> >",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "allocator<std::tr1::_Impl_no_alloc1<std::tr1::_Callable_obj<<lambda162>,0>,void,COnlineQueryResult const &> >"
        ]
    );
}

#[test]
fn test_type_621() {
    let result = parse_type(
        "struct __cppobj std::allocator<std::tr1::_Impl_no_alloc1<std::tr1::_Callable_obj<<lambda173>,0>,void,CCallContext &> > : std::_Allocator_base<std::tr1::_Impl_no_alloc1<std::tr1::_Callable_obj<<lambda173>,0>,void,CCallContext &> >",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "allocator<std::tr1::_Impl_no_alloc1<std::tr1::_Callable_obj<<lambda173>,0>,void,CCallContext &> >"
        ]
    );
}

#[test]
fn test_type_622() {
    let result = parse_type(
        "struct __cppobj std::allocator<std::tr1::_Impl_no_alloc1<std::tr1::_Callable_obj<<lambda79>,0>,void,COnlineQueryResult const &> > : std::_Allocator_base<std::tr1::_Impl_no_alloc1<std::tr1::_Callable_obj<<lambda79>,0>,void,COnlineQueryResult const &> >",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "allocator<std::tr1::_Impl_no_alloc1<std::tr1::_Callable_obj<<lambda79>,0>,void,COnlineQueryResult const &> >"
        ]
    );
}

#[test]
fn test_type_623() {
    let result = parse_type(
        "struct __cppobj std::allocator<std::tr1::_Impl_no_alloc1<std::tr1::_Callable_obj<NGraphicsEngine::<lambda20>,0>,void,float const *> > : std::_Allocator_base<std::tr1::_Impl_no_alloc1<std::tr1::_Callable_obj<NGraphicsEngine::<lambda20>,0>,void,float const *> >",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "allocator<std::tr1::_Impl_no_alloc1<std::tr1::_Callable_obj<NGraphicsEngine::<lambda20>,0>,void,float const *> >"
        ]
    );
}

#[test]
fn test_type_624() {
    let result = parse_type(
        "struct __cppobj std::allocator<std::tr1::function<void __cdecl(CCallContextResult<boost::shared_ptr<CFriend const > > &)> >::rebind<std::tr1::function<void __cdecl(CCallContextResult<boost::shared_ptr<CFriend const > > &)> >",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "allocator<std::tr1::function<void __cdecl(CCallContextResult<boost::shared_ptr<CFriend const > > &)> >",
            "rebind<std::tr1::function<void __cdecl(CCallContextResult<boost::shared_ptr<CFriend const > > &)> >"
        ]
    );
}

#[test]
fn test_type_625() {
    let result =
        parse_type("struct __cppobj std::back_insert_iterator<std::vector<int> > : std::_Outit");
    assert_eq!(
        result,
        vec!["std", "back_insert_iterator<std::vector<int> >"]
    );
}

#[test]
fn test_type_626() {
    let result = parse_type(
        "struct __cppobj std::default_delete<NGraphScript::CPersistentVariableEventHandler>",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "default_delete<NGraphScript::CPersistentVariableEventHandler>"
        ]
    );
}

#[test]
fn test_type_627() {
    let result = parse_type(
        "struct __cppobj std::insert_iterator<boost::unordered::unordered_map<unsigned int,float const *,boost::hash<unsigned int>,std::equal_to<unsigned int>,std::allocator<std::pair<unsigned int const ,float const *> > > > : std::_Outit",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "insert_iterator<boost::unordered::unordered_map<unsigned int,float const *,boost::hash<unsigned int>,std::equal_to<unsigned int>,std::allocator<std::pair<unsigned int const ,float const *> > > >"
        ]
    );
}

#[test]
fn test_type_628() {
    let result = parse_type(
        "struct __cppobj std::iterator<std::forward_iterator_tag,std::pair<std::string const ,float>,__int64,boost::unordered::detail::ptr_node<std::pair<std::string const ,float> > *,std::pair<std::string const ,float> &>",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "iterator<std::forward_iterator_tag,std::pair<std::string const ,float>,__int64,boost::unordered::detail::ptr_node<std::pair<std::string const ,float> > *,std::pair<std::string const ,float> &>"
        ]
    );
}

#[test]
fn test_type_629() {
    let result = parse_type(
        "struct __cppobj std::less_equal<void *> : std::binary_function<void *,void *,bool>",
    );
    assert_eq!(result, vec!["std", "less_equal<void *>"]);
}

#[test]
fn test_type_630() {
    let result = parse_type(
        "struct __cppobj std::list<std::pair<NRevolution::SRegionRecord const * const,std::string >> : std::_List_val<std::pair<NRevolution::SRegionRecord const * const,std::string >>",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "list<std::pair<NRevolution::SRegionRecord const * const,std::string >>"
        ]
    );
}

#[test]
fn test_type_631() {
    let result = parse_type(
        "struct __cppobj std::map<std::string,std::vector<CColorRGBAb>> : std::_Tree<std::_Tmap_traits<std::string,std::vector<CColorRGBAb>,std::less<std::string >,std::allocator<std::pair<std::string const ,std::vector<CColorRGBAb> > >,0> >",
    );
    assert_eq!(
        result,
        vec!["std", "map<std::string,std::vector<CColorRGBAb>>"]
    );
}

#[test]
fn test_type_632() {
    let result = parse_type(
        "struct __cppobj std::pair<boost::container::container_detail::vec_iterator<boost::container::container_detail::pair<CHashString,STaskInfo> *,0>,bool> : std::_Pair_base<boost::container::container_detail::vec_iterator<boost::container::container_detail::pair<CHashString,STaskInfo> *,0>,bool>",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "pair<boost::container::container_detail::vec_iterator<boost::container::container_detail::pair<CHashString,STaskInfo> *,0>,bool>"
        ]
    );
}

#[test]
fn test_type_633() {
    let result = parse_type(
        "struct __cppobj std::pair<boost::container::container_detail::vec_iterator<boost::container::container_detail::pair<CHashString,std::vector<std::vector<CCollectionManager::SCollectibleRecord>> > *,0>,bool> : std::_Pair_base<boost::container::container_detail::vec_iterator<boost::container::container_detail::pair<CHashString,std::vector<std::vector<CCollectionManager::SCollectibleRecord>> > *,0>,bool>",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "pair<boost::container::container_detail::vec_iterator<boost::container::container_detail::pair<CHashString,std::vector<std::vector<CCollectionManager::SCollectibleRecord>> > *,0>,bool>"
        ]
    );
}

#[test]
fn test_type_634() {
    let result = parse_type(
        "struct __cppobj std::pair<CStatisticContext,__int64> : std::_Pair_base<CStatisticContext,__int64>",
    );
    assert_eq!(result, vec!["std", "pair<CStatisticContext,__int64>"]);
}

#[test]
fn test_type_635() {
    let result = parse_type(
        "struct __cppobj std::pair<std::_Tree_const_iterator<std::_Tree_val<std::_Tset_traits<boost::weak_ptr<CGameObject>,std::less<boost::weak_ptr<CGameObject> >,std::allocator<boost::weak_ptr<CGameObject> >,0> > >,bool> : std::_Pair_base<std::_Tree_const_iterator<std::_Tree_val<std::_Tset_traits<boost::weak_ptr<CGameObject>,std::less<boost::weak_ptr<CGameObject> >,std::allocator<boost::weak_ptr<CGameObject> >,0> > >,bool>",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "pair<std::_Tree_const_iterator<std::_Tree_val<std::_Tset_traits<boost::weak_ptr<CGameObject>,std::less<boost::weak_ptr<CGameObject> >,std::allocator<boost::weak_ptr<CGameObject> >,0> > >,bool>"
        ]
    );
}

#[test]
fn test_type_636() {
    let result = parse_type(
        "struct __cppobj std::pair<std::_Tree_iterator<std::_Tree_val<std::_Tmap_traits<std::string,std::string,std::less<std::string >,std::allocator<std::pair<std::string const ,std::string > >,0> > >,bool> : std::_Pair_base<std::_Tree_iterator<std::_Tree_val<std::_Tmap_traits<std::string,std::string,std::less<std::string >,std::allocator<std::pair<std::string const ,std::string > >,0> > >,bool>",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "pair<std::_Tree_iterator<std::_Tree_val<std::_Tmap_traits<std::string,std::string,std::less<std::string >,std::allocator<std::pair<std::string const ,std::string > >,0> > >,bool>"
        ]
    );
}

#[test]
fn test_type_637() {
    let result = parse_type(
        "struct __cppobj std::pair<std::string const ,float> : std::_Pair_base<std::string const ,float>",
    );
    assert_eq!(result, vec!["std", "pair<std::string const ,float>"]);
}

#[test]
fn test_type_638() {
    let result = parse_type(
        "struct __cppobj std::pair<unsigned int const ,std::unique_ptr<NAttachedEffectContainer::SUsageInstanceDebugInfo> > : std::_Pair_base<unsigned int const ,std::unique_ptr<NAttachedEffectContainer::SUsageInstanceDebugInfo> >",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "pair<unsigned int const ,std::unique_ptr<NAttachedEffectContainer::SUsageInstanceDebugInfo> >"
        ]
    );
}

#[test]
fn test_type_639() {
    let result = parse_type(
        "struct __cppobj std::set<CTarget *> : std::_Tree<std::_Tset_traits<CTarget *,std::less<CTarget *>,std::allocator<CTarget *>,0> >",
    );
    assert_eq!(result, vec!["std", "set<CTarget *>"]);
}

#[test]
fn test_type_640() {
    let result = parse_type(
        "struct __cppobj std::tr1::_Bind1<std::tr1::_Callable_pmf<void (__cdecl NSaveLoad::CManagedObject<NSaveLoad::CLocalDataRef<NSaveLoad::Test::UserType> >::*const)(void),NSaveLoad::CManagedObject<NSaveLoad::CLocalDataRef<NSaveLoad::Test::UserType> >,0>,NSaveLoad::CManagedObject<NSaveLoad::CLocalDataRef<NSaveLoad::Test::UserType> > *>::_Return<std::tr1::_Nil &,std::tr1::_Nil &,std::tr1::_Nil &,std::tr1::_Nil &,std::tr1::_Nil &,std::tr1::_Nil &,std::tr1::_Nil &,std::tr1::_Nil &,std::tr1::_Nil &,std::tr1::_Nil &>",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "tr1",
            "_Bind1<std::tr1::_Callable_pmf<void (__cdecl NSaveLoad::CManagedObject<NSaveLoad::CLocalDataRef<NSaveLoad::Test::UserType> >::*const)(void),NSaveLoad::CManagedObject<NSaveLoad::CLocalDataRef<NSaveLoad::Test::UserType> >,0>,NSaveLoad::CManagedObject<NSaveLoad::CLocalDataRef<NSaveLoad::Test::UserType> > *>",
            "_Return<std::tr1::_Nil &,std::tr1::_Nil &,std::tr1::_Nil &,std::tr1::_Nil &,std::tr1::_Nil &,std::tr1::_Nil &,std::tr1::_Nil &,std::tr1::_Nil &,std::tr1::_Nil &,std::tr1::_Nil &>"
        ]
    );
}

#[test]
fn test_type_641() {
    let result = parse_type("struct __cppobj std::tr1::_Callable_base<<lambda16>,0>");
    assert_eq!(result, vec!["std", "tr1", "_Callable_base<<lambda16>,0>"]);
}

#[test]
fn test_type_642() {
    let result = parse_type("struct __cppobj std::tr1::_Callable_base<<lambda20>,0>");
    assert_eq!(result, vec!["std", "tr1", "_Callable_base<<lambda20>,0>"]);
}

#[test]
fn test_type_643() {
    let result = parse_type("struct __cppobj std::tr1::_Callable_base<A0x2c6d2d03::<lambda42>,0>");
    assert_eq!(
        result,
        vec!["std", "tr1", "_Callable_base<A0x2c6d2d03::<lambda42>,0>"]
    );
}

#[test]
fn test_type_644() {
    let result = parse_type(
        "struct __cppobj std::tr1::_Callable_base<std::tr1::_Bind<void,void,std::tr1::_Bind1<std::tr1::_Callable_pmf<void (__cdecl CPfxVehicle::*const)(void),CPfxVehicle,0>,CPfxVehicle *> >,0>",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "tr1",
            "_Callable_base<std::tr1::_Bind<void,void,std::tr1::_Bind1<std::tr1::_Callable_pmf<void (__cdecl CPfxVehicle::*const)(void),CPfxVehicle,0>,CPfxVehicle *> >,0>"
        ]
    );
}

#[test]
fn test_type_645() {
    let result = parse_type(
        "struct __cppobj std::tr1::_Callable_obj<<lambda160>,0> : std::tr1::_Callable_base<<lambda160>,0>",
    );
    assert_eq!(result, vec!["std", "tr1", "_Callable_obj<<lambda160>,0>"]);
}

#[test]
fn test_type_646() {
    let result = parse_type(
        "struct __cppobj std::tr1::_Callable_obj<<lambda30>,0> : std::tr1::_Callable_base<<lambda30>,0>",
    );
    assert_eq!(result, vec!["std", "tr1", "_Callable_obj<<lambda30>,0>"]);
}

#[test]
fn test_type_647() {
    let result = parse_type(
        "struct __cppobj std::tr1::_Callable_obj<<lambda89>,0> : std::tr1::_Callable_base<<lambda89>,0>",
    );
    assert_eq!(result, vec!["std", "tr1", "_Callable_obj<<lambda89>,0>"]);
}

#[test]
fn test_type_648() {
    let result = parse_type(
        "struct __cppobj std::tr1::_Callable_obj<NMissionSystem::<lambda12>,0> : std::tr1::_Callable_base<NMissionSystem::<lambda12>,0>",
    );
    assert_eq!(
        result,
        vec!["std", "tr1", "_Callable_obj<NMissionSystem::<lambda12>,0>"]
    );
}

#[test]
fn test_type_649() {
    let result = parse_type(
        "struct __cppobj std::tr1::_Callable_obj<std::tr1::_Bind<void,void,std::tr1::_Bind1<std::tr1::_Callable_pmf<void (__cdecl NSaveLoad::CManagedObject<NSaveLoad::CRemoteDataObject<NSaveLoad::Test::UserType,NSaveLoad::DefaultObjectFactory> >::*const)(void),NSaveLoad::CManagedObject<NSaveLoad::CRemoteDataObject<NSaveLoad::Test::UserType,NSaveLoad::DefaultObjectFactory> >,0>,NSaveLoad::CManagedObject<NSaveLoad::CRemoteDataObject<NSaveLoad::Test::UserType,NSaveLoad::DefaultObjectFactory> > *> >,0> : std::tr1::_Callable_base<std::tr1::_Bind<void,void,std::tr1::_Bind1<std::tr1::_Callable_pmf<void (__cdecl NSaveLoad::CManagedObject<NSaveLoad::CRemoteDataObject<NSaveLoad::Test::UserType,NSaveLoad::DefaultObjectFactory> >::*const)(void),NSaveLoad::CManagedObject<NSaveLoad::CRemoteDataObject<NSaveLoad::Test::UserType,NSaveLoad::DefaultObjectFactory> >,0>,NSaveLoad::CManagedObject<NSaveLoad::CRemoteDataObject<NSaveLoad::Test::UserType,NSaveLoad::DefaultObjectFactory> > *> >,0>",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "tr1",
            "_Callable_obj<std::tr1::_Bind<void,void,std::tr1::_Bind1<std::tr1::_Callable_pmf<void (__cdecl NSaveLoad::CManagedObject<NSaveLoad::CRemoteDataObject<NSaveLoad::Test::UserType,NSaveLoad::DefaultObjectFactory> >::*const)(void),NSaveLoad::CManagedObject<NSaveLoad::CRemoteDataObject<NSaveLoad::Test::UserType,NSaveLoad::DefaultObjectFactory> >,0>,NSaveLoad::CManagedObject<NSaveLoad::CRemoteDataObject<NSaveLoad::Test::UserType,NSaveLoad::DefaultObjectFactory> > *> >,0>"
        ]
    );
}

#[test]
fn test_type_650() {
    let result = parse_type(
        "struct __cppobj std::tr1::_Function_impl1<void,CCallContextResult<boost::shared_ptr<CFriend const > > &> : std::unary_function<CCallContextResult<boost::shared_ptr<CFriend const > > &,void>",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "tr1",
            "_Function_impl1<void,CCallContextResult<boost::shared_ptr<CFriend const > > &>"
        ]
    );
}

#[test]
fn test_type_651() {
    let result = parse_type(
        "struct __cppobj std::tr1::_Function_impl2<void,std::unique_ptr<char [0]> const &,unsigned __int64 const &> : std::binary_function<std::unique_ptr<char [0]> const &,unsigned __int64 const &,void>",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "tr1",
            "_Function_impl2<void,std::unique_ptr<char [0]> const &,unsigned __int64 const &>"
        ]
    );
}

#[test]
fn test_type_652() {
    let result = parse_type(
        "struct __cppobj std::tr1::_Impl_base2<void,std::string const &,std::string const &>",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "tr1",
            "_Impl_base2<void,std::string const &,std::string const &>"
        ]
    );
}

#[test]
fn test_type_653() {
    let result = parse_type(
        "struct __cppobj std::tr1::_Impl_no_alloc0<std::tr1::_Callable_obj<NPfxBreakable::SDeferredAngularImpulseApplicator_World<CPfxBreakable,1724575678>,0>,void> : std::tr1::_Impl_base0<void>",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "tr1",
            "_Impl_no_alloc0<std::tr1::_Callable_obj<NPfxBreakable::SDeferredAngularImpulseApplicator_World<CPfxBreakable,1724575678>,0>,void>"
        ]
    );
}

#[test]
fn test_type_654() {
    let result = parse_type(
        "struct __cppobj std::tr1::_Impl_no_alloc0<std::tr1::_Callable_obj<NPfxBreakable::SDeferredImpulseAtPointApplicator_World<CPfxBreakable,1482107661>,0>,void> : std::tr1::_Impl_base0<void>",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "tr1",
            "_Impl_no_alloc0<std::tr1::_Callable_obj<NPfxBreakable::SDeferredImpulseAtPointApplicator_World<CPfxBreakable,1482107661>,0>,void>"
        ]
    );
}

#[test]
fn test_type_655() {
    let result = parse_type(
        "struct __cppobj std::tr1::_Impl_no_alloc1<std::tr1::_Callable_obj<<lambda106>,0>,void,CCallContext &> : std::tr1::_Impl_base1<void,CCallContext &>",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "tr1",
            "_Impl_no_alloc1<std::tr1::_Callable_obj<<lambda106>,0>,void,CCallContext &>"
        ]
    );
}

#[test]
fn test_type_656() {
    let result = parse_type(
        "struct __cppobj std::tr1::_Impl_no_alloc1<std::tr1::_Callable_obj<<lambda180>,0>,void,CCallContext &> : std::tr1::_Impl_base1<void,CCallContext &>",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "tr1",
            "_Impl_no_alloc1<std::tr1::_Callable_obj<<lambda180>,0>,void,CCallContext &>"
        ]
    );
}

#[test]
fn test_type_657() {
    let result = parse_type(
        "struct __cppobj std::tr1::_Impl_no_alloc1<std::tr1::_Callable_obj<<lambda19>,0>,void,float const *> : std::tr1::_Impl_base1<void,float const *>",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "tr1",
            "_Impl_no_alloc1<std::tr1::_Callable_obj<<lambda19>,0>,void,float const *>"
        ]
    );
}

#[test]
fn test_type_658() {
    let result = parse_type(
        "struct __cppobj std::tr1::_Impl_no_alloc1<std::tr1::_Callable_obj<A0x743c4eea::<lambda225>,0>,CHashString,unsigned int> : std::tr1::_Impl_base1<CHashString,unsigned int>",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "tr1",
            "_Impl_no_alloc1<std::tr1::_Callable_obj<A0x743c4eea::<lambda225>,0>,CHashString,unsigned int>"
        ]
    );
}

#[test]
fn test_type_659() {
    let result = parse_type(
        "struct __cppobj std::tr1::_Impl_no_alloc1<std::tr1::_Callable_obj<CLoginDataLoader::OnLoadComplete,0>,void,CCallContext &> : std::tr1::_Impl_base1<void,CCallContext &>",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "tr1",
            "_Impl_no_alloc1<std::tr1::_Callable_obj<CLoginDataLoader::OnLoadComplete,0>,void,CCallContext &>"
        ]
    );
}

#[test]
fn test_type_660() {
    let result = parse_type(
        "struct __cppobj std::tr1::_Impl_no_alloc1<std::tr1::_Callable_obj<NGraphicsEngine::<lambda14>,0>,void,float const *> : std::tr1::_Impl_base1<void,float const *>",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "tr1",
            "_Impl_no_alloc1<std::tr1::_Callable_obj<NGraphicsEngine::<lambda14>,0>,void,float const *>"
        ]
    );
}

#[test]
fn test_type_661() {
    let result = parse_type(
        "struct __cppobj std::tr1::_Impl_no_alloc1<std::tr1::_Callable_obj<NGraphicsEngine::<lambda8>,0>,void,float const *> : std::tr1::_Impl_base1<void,float const *>",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "tr1",
            "_Impl_no_alloc1<std::tr1::_Callable_obj<NGraphicsEngine::<lambda8>,0>,void,float const *>"
        ]
    );
}

#[test]
fn test_type_662() {
    let result = parse_type(
        "struct __cppobj std::tr1::_Umap_traits<CHashString,bool,std::_Hash_compare<CHashString,std::hash<CHashString>,std::equal_to<CHashString> >,std::allocator<std::pair<CHashString const ,bool> >,0> : std::_Container_base0",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "tr1",
            "_Umap_traits<CHashString,bool,std::_Hash_compare<CHashString,std::hash<CHashString>,std::equal_to<CHashString> >,std::allocator<std::pair<CHashString const ,bool> >,0>"
        ]
    );
}

#[test]
fn test_type_663() {
    let result = parse_type(
        "struct __cppobj std::tr1::_Umap_traits<PlayerId_Steam,CUser *,std::_Hash_compare<PlayerId_Steam,std::hash<PlayerId_Steam>,std::equal_to<PlayerId_Steam> >,std::allocator<std::pair<PlayerId_Steam const ,CUser *> >,0> : std::_Container_base0",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "tr1",
            "_Umap_traits<PlayerId_Steam,CUser *,std::_Hash_compare<PlayerId_Steam,std::hash<PlayerId_Steam>,std::equal_to<PlayerId_Steam> >,std::allocator<std::pair<PlayerId_Steam const ,CUser *> >,0>"
        ]
    );
}

#[test]
fn test_type_664() {
    let result = parse_type("struct __cppobj std::tr1::bad_weak_ptr : std::exception");
    assert_eq!(result, vec!["std", "tr1", "bad_weak_ptr"]);
}

#[test]
fn test_type_665() {
    let result = parse_type(
        "struct __cppobj std::tr1::unordered_map<unsigned int,CCommBragsFeatsUI::SSelectedItem,std::hash<unsigned int>,std::equal_to<unsigned int>,std::allocator<std::pair<unsigned int const ,CCommBragsFeatsUI::SSelectedItem> > > : std::_Hash<std::tr1::_Umap_traits<unsigned int,CCommBragsFeatsUI::SSelectedItem,std::_Hash_compare<unsigned int,std::hash<unsigned int>,std::equal_to<unsigned int> >,std::allocator<std::pair<unsigned int const ,CCommBragsFeatsUI::SSelectedItem> >,0> >",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "tr1",
            "unordered_map<unsigned int,CCommBragsFeatsUI::SSelectedItem,std::hash<unsigned int>,std::equal_to<unsigned int>,std::allocator<std::pair<unsigned int const ,CCommBragsFeatsUI::SSelectedItem> > >"
        ]
    );
}

#[test]
fn test_type_666() {
    let result = parse_type(
        "struct __cppobj std::unique_ptr<NAttachedEffectContainer::SUsageInstanceDebugInfo> : std::_Unique_ptr_base<NAttachedEffectContainer::SUsageInstanceDebugInfo,std::default_delete<NAttachedEffectContainer::SUsageInstanceDebugInfo>,1>",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "unique_ptr<NAttachedEffectContainer::SUsageInstanceDebugInfo>"
        ]
    );
}

#[test]
fn test_type_667() {
    let result = parse_type(
        "struct __cppobj std::vector<CCharacter::STetherAttachment> : std::_Vector_val<CCharacter::STetherAttachment>",
    );
    assert_eq!(result, vec!["std", "vector<CCharacter::STetherAttachment>"]);
}

#[test]
fn test_type_668() {
    let result = parse_type(
        "struct __cppobj std::vector<CStatisticMarshaler::SRegisterInfo<TDelegate<void __cdecl(CStatistic<float> const *,float,float,CStatisticContext const &,void *)> >> : std::_Vector_val<CStatisticMarshaler::SRegisterInfo<TDelegate<void __cdecl(CStatistic<float> const *,float,float,CStatisticContext const &,void *)> >>",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "vector<CStatisticMarshaler::SRegisterInfo<TDelegate<void __cdecl(CStatistic<float> const *,float,float,CStatisticContext const &,void *)> >>"
        ]
    );
}

#[test]
fn test_type_669() {
    let result = parse_type(
        "struct __cppobj std::vector<NGraphicsEngine::SScopeIndexer> : std::_Vector_val<NGraphicsEngine::SScopeIndexer>",
    );
    assert_eq!(
        result,
        vec!["std", "vector<NGraphicsEngine::SScopeIndexer>"]
    );
}

#[test]
fn test_type_670() {
    let result = parse_type(
        "struct __cppobj std::vector<NVehicle_Debug::SDestroyedVehicleLog> : std::_Vector_val<NVehicle_Debug::SDestroyedVehicleLog>",
    );
    assert_eq!(
        result,
        vec!["std", "vector<NVehicle_Debug::SDestroyedVehicleLog>"]
    );
}

#[test]
fn test_type_671() {
    let result = parse_type(
        "struct __cppobj std::vector<SAttemptContext> : std::_Vector_val<SAttemptContext>",
    );
    assert_eq!(result, vec!["std", "vector<SAttemptContext>"]);
}

#[test]
fn test_type_672() {
    let result = parse_type(
        "struct __cppobj std::vector<TResourceCachePtr<SCollection>> : std::_Vector_val<TResourceCachePtr<SCollection>>",
    );
    assert_eq!(
        result,
        vec!["std", "vector<TResourceCachePtr<SCollection>>"]
    );
}

#[test]
fn test_type_673() {
    let result =
        parse_type("struct __cppobj Steam::HTML_OpenLinkInNewTab_t : Steam::SteamCallback_t");
    assert_eq!(result, vec!["Steam", "HTML_OpenLinkInNewTab_t"]);
}

#[test]
fn test_type_674() {
    let result = parse_type("struct __cppobj STerrainStreamPatchDataInternal");
    assert_eq!(result, vec!["STerrainStreamPatchDataInternal"]);
}

#[test]
fn test_type_675() {
    let result = parse_type("struct __cppobj SVfsArchive");
    assert_eq!(result, vec!["SVfsArchive"]);
}

#[test]
fn test_type_676() {
    let result = parse_type("struct __cppobj SVfsHttpArchiveProviderInstance");
    assert_eq!(result, vec!["SVfsHttpArchiveProviderInstance"]);
}

#[test]
fn test_type_677() {
    let result = parse_type("struct __cppobj TAdfStructPtr<SDeformPoints> : CAdfStructPtrBase");
    assert_eq!(result, vec!["TAdfStructPtr<SDeformPoints>"]);
}

#[test]
fn test_type_678() {
    let result = parse_type("struct __cppobj TAdfStructPtr<SSubmersible> : CAdfStructPtrBase");
    assert_eq!(result, vec!["TAdfStructPtr<SSubmersible>"]);
}

#[test]
fn test_type_679() {
    let result = parse_type(
        "struct __cppobj TaskQueue::detail::CSyncWork<NSaveLoad::<lambda6>,enum NOnline::EErrorCode,void> : TaskQueue::detail::CWork<NSaveLoad::<lambda6>,enum NOnline::EErrorCode,void>",
    );
    assert_eq!(
        result,
        vec![
            "TaskQueue",
            "detail",
            "CSyncWork<NSaveLoad::<lambda6>,enum NOnline::EErrorCode,void>"
        ]
    );
}

#[test]
fn test_type_680() {
    let result = parse_type(
        "struct __cppobj TaskQueue::detail::CSyncWork<NSaveLoad::<lambda9>,void,bool> : TaskQueue::detail::CWork<NSaveLoad::<lambda9>,void,bool>",
    );
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
fn test_type_681() {
    let result = parse_type("struct __cppobj TaskQueue::detail::IExecutor");
    assert_eq!(result, vec!["TaskQueue", "detail", "IExecutor"]);
}

#[test]
fn test_type_682() {
    let result = parse_type(
        "struct __cppobj testing::internal::is_pointer<unsigned __int64> : testing::internal::bool_constant<0>",
    );
    assert_eq!(
        result,
        vec!["testing", "internal", "is_pointer<unsigned __int64>"]
    );
}

#[test]
fn test_type_683() {
    let result = parse_type(
        "struct __cppobj TIterableItemPool<CLightInstantiator,16>::iterator : TIterableItemPool<CLightInstantiator,16>::const_iterator",
    );
    assert_eq!(
        result,
        vec!["TIterableItemPool<CLightInstantiator,16>", "iterator"]
    );
}

#[test]
fn test_type_684() {
    let result = parse_type(
        "struct __cppobj TPropertyContainer<ava::idstring<PropertyContainerIdStringTag,1>,std::allocator<int>,CIDStringHash>",
    );
    assert_eq!(
        result,
        vec![
            "TPropertyContainer<ava::idstring<PropertyContainerIdStringTag,1>,std::allocator<int>,CIDStringHash>"
        ]
    );
}

#[test]
fn test_type_685() {
    let result = parse_type("struct __cppobj ValueTypeTrait<1917069251>");
    assert_eq!(result, vec!["ValueTypeTrait<1917069251>"]);
}

#[test]
fn test_type_686() {
    let result = parse_type("struct __cppobj ValueTypeTrait<2728888050>");
    assert_eq!(result, vec!["ValueTypeTrait<2728888050>"]);
}

#[test]
fn test_type_687() {
    let result = parse_type("struct __cppobj ValueTypeTrait<524657564>");
    assert_eq!(result, vec!["ValueTypeTrait<524657564>"]);
}

#[test]
fn test_type_688() {
    let result = parse_type(
        "struct __declspec(align(2)) hknpManifoldCollisionCache::<unnamed_type_m_scratch2>::CvxCvx",
    );
    assert_eq!(
        result,
        vec![
            "hknpManifoldCollisionCache",
            "<unnamed_type_m_scratch2>",
            "CvxCvx"
        ]
    );
}

#[test]
fn test_type_689() {
    let result = parse_type("struct __declspec(align(4)) SHintModifierParams");
    assert_eq!(result, vec!["SHintModifierParams"]);
}

#[test]
fn test_type_690() {
    let result = parse_type("struct __declspec(align(8)) Graphics::SBeginCommandBufferParams");
    assert_eq!(result, vec!["Graphics", "SBeginCommandBufferParams"]);
}

#[test]
fn test_type_691() {
    let result = parse_type(
        "struct __declspec(align(8)) hkGeometryProcessing::PoolAllocator<hkgpIndexedMeshDefinitions::Triangle,32,hkContainerHeapAllocator>::Pool",
    );
    assert_eq!(
        result,
        vec![
            "hkGeometryProcessing",
            "PoolAllocator<hkgpIndexedMeshDefinitions::Triangle,32,hkContainerHeapAllocator>",
            "Pool"
        ]
    );
}

#[test]
fn test_type_692() {
    let result = parse_type("struct __declspec(align(8)) hkSolverAllocator::Element");
    assert_eq!(result, vec!["hkSolverAllocator", "Element"]);
}

#[test]
fn test_type_693() {
    let result =
        parse_type("struct __declspec(align(8)) SAttachSettings::Arrayplayer_attach_setting");
    assert_eq!(
        result,
        vec!["SAttachSettings", "Arrayplayer_attach_setting"]
    );
}

#[test]
fn test_type_694() {
    let result = parse_type("struct __declspec(align(8)) SDecals::Arraydecal_counts");
    assert_eq!(result, vec!["SDecals", "Arraydecal_counts"]);
}

#[test]
fn test_type_695() {
    let result = parse_type("struct __declspec(align(8)) SEffectRTParamHandler::ArrayParamHash");
    assert_eq!(result, vec!["SEffectRTParamHandler", "ArrayParamHash"]);
}

#[test]
fn test_type_696() {
    let result =
        parse_type("struct __declspec(align(8)) SPartEffectReactions::Arrayon_part_burning");
    assert_eq!(result, vec!["SPartEffectReactions", "Arrayon_part_burning"]);
}

#[test]
fn test_type_697() {
    let result = parse_type("struct __declspec(align(8)) STerrainSystem::ArrayShaderRules");
    assert_eq!(result, vec!["STerrainSystem", "ArrayShaderRules"]);
}

#[test]
fn test_type_698() {
    let result =
        parse_type("struct __declspec(align(8)) SUsageEffectAttachments::Arrayeffect_indices");
    assert_eq!(
        result,
        vec!["SUsageEffectAttachments", "Arrayeffect_indices"]
    );
}

#[test]
fn test_type_699() {
    let result =
        parse_type("struct __unaligned __declspec(align(1)) $_TypeDescriptor$_extraBytes_161");
    assert_eq!(result, vec!["$_TypeDescriptor$_extraBytes_161"]);
}

#[test]
fn test_type_700() {
    let result =
        parse_type("struct __unaligned __declspec(align(1)) $_TypeDescriptor$_extraBytes_247");
    assert_eq!(result, vec!["$_TypeDescriptor$_extraBytes_247"]);
}

#[test]
fn test_type_701() {
    let result =
        parse_type("struct __unaligned __declspec(align(1)) $_TypeDescriptor$_extraBytes_345");
    assert_eq!(result, vec!["$_TypeDescriptor$_extraBytes_345"]);
}

#[test]
fn test_type_702() {
    let result = parse_type("struct __unaligned __declspec(align(4)) XAUDIO2_VOICE_STATE");
    assert_eq!(result, vec!["XAUDIO2_VOICE_STATE"]);
}

#[test]
fn test_type_703() {
    let result = parse_type("struct _DLLVERSIONINFO");
    assert_eq!(result, vec!["_DLLVERSIONINFO"]);
}

#[test]
fn test_type_704() {
    let result = parse_type("struct _IMAGE_BOUND_IMPORT_DESCRIPTOR");
    assert_eq!(result, vec!["_IMAGE_BOUND_IMPORT_DESCRIPTOR"]);
}

#[test]
fn test_type_705() {
    let result = parse_type("struct _REMOTE_NAME_INFOW");
    assert_eq!(result, vec!["_REMOTE_NAME_INFOW"]);
}

#[test]
fn test_type_706() {
    let result = parse_type("struct _STARTUPINFOW");
    assert_eq!(result, vec!["_STARTUPINFOW"]);
}

#[test]
fn test_type_707() {
    let result = parse_type("struct _SYSTEM_LOGICAL_PROCESSOR_INFORMATION");
    assert_eq!(result, vec!["_SYSTEM_LOGICAL_PROCESSOR_INFORMATION"]);
}

#[test]
fn test_type_708() {
    let result = parse_type(
        "struct /*VFT*/ boost::concepts::check<boost::concepts::usage_requirements<boost::range_detail::IncrementableIteratorConcept<std::_String_iterator<char,std::char_traits<char>,std::allocator<char> > > > >_vtbl",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "concepts",
            "check<boost::concepts::usage_requirements<boost::range_detail::IncrementableIteratorConcept<std::_String_iterator<char,std::char_traits<char>,std::allocator<char> > > > >_vtbl"
        ]
    );
}

#[test]
fn test_type_709() {
    let result = parse_type(
        "struct /*VFT*/ boost::detail::sp_counted_impl_pd<CAITrafficZone *,CGameObjectCreator<CAITrafficZone>::SGameObjectDestroyer>_vtbl",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "detail",
            "sp_counted_impl_pd<CAITrafficZone *,CGameObjectCreator<CAITrafficZone>::SGameObjectDestroyer>_vtbl"
        ]
    );
}

#[test]
fn test_type_710() {
    let result = parse_type(
        "struct /*VFT*/ boost::detail::sp_counted_impl_pd<CContextActionScene *,CGameObjectCreator<CContextActionScene>::SGameObjectDestroyer>_vtbl",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "detail",
            "sp_counted_impl_pd<CContextActionScene *,CGameObjectCreator<CContextActionScene>::SGameObjectDestroyer>_vtbl"
        ]
    );
}

#[test]
fn test_type_711() {
    let result = parse_type(
        "struct /*VFT*/ boost::detail::sp_counted_impl_pd<CEncounterObject *,CGameObjectCreator<CEncounterObject>::SGameObjectDestroyer>_vtbl",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "detail",
            "sp_counted_impl_pd<CEncounterObject *,CGameObjectCreator<CEncounterObject>::SGameObjectDestroyer>_vtbl"
        ]
    );
}

#[test]
fn test_type_712() {
    let result = parse_type(
        "struct /*VFT*/ boost::detail::sp_counted_impl_pd<NSaveLoad::CUserDataStream *,boost::detail::sp_ms_deleter<NSaveLoad::CUserDataStream> >_vtbl",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "detail",
            "sp_counted_impl_pd<NSaveLoad::CUserDataStream *,boost::detail::sp_ms_deleter<NSaveLoad::CUserDataStream> >_vtbl"
        ]
    );
}

#[test]
fn test_type_713() {
    let result = parse_type("struct /*VFT*/ CActionTokenManager_vtbl");
    assert_eq!(result, vec!["CActionTokenManager_vtbl"]);
}

#[test]
fn test_type_714() {
    let result = parse_type("struct /*VFT*/ CAdditiveLookAtBlendState_vtbl");
    assert_eq!(result, vec!["CAdditiveLookAtBlendState_vtbl"]);
}

#[test]
fn test_type_715() {
    let result = parse_type("struct /*VFT*/ CAiAreaOfOperationsEntity_vtbl");
    assert_eq!(result, vec!["CAiAreaOfOperationsEntity_vtbl"]);
}

#[test]
fn test_type_716() {
    let result = parse_type("struct /*VFT*/ CAppSystemCreator<CTargetSystem>_vtbl");
    assert_eq!(result, vec!["CAppSystemCreator<CTargetSystem>_vtbl"]);
}

#[test]
fn test_type_717() {
    let result = parse_type("struct /*VFT*/ CAsynchronousBragManager_vtbl");
    assert_eq!(result, vec!["CAsynchronousBragManager_vtbl"]);
}

#[test]
fn test_type_718() {
    let result = parse_type("struct /*VFT*/ CBaseNetworkComponentManager_vtbl");
    assert_eq!(result, vec!["CBaseNetworkComponentManager_vtbl"]);
}

#[test]
fn test_type_719() {
    let result = parse_type("struct /*VFT*/ CCommBragsFeatsUI_vtbl");
    assert_eq!(result, vec!["CCommBragsFeatsUI_vtbl"]);
}

#[test]
fn test_type_720() {
    let result = parse_type("struct /*VFT*/ CCompareCondition_vtbl");
    assert_eq!(result, vec!["CCompareCondition_vtbl"]);
}

#[test]
fn test_type_721() {
    let result = parse_type("struct /*VFT*/ CFadeInCommand_vtbl");
    assert_eq!(result, vec!["CFadeInCommand_vtbl"]);
}

#[test]
fn test_type_722() {
    let result = parse_type("struct /*VFT*/ CGameObjectCreator<CInteractionContext>_vtbl");
    assert_eq!(result, vec!["CGameObjectCreator<CInteractionContext>_vtbl"]);
}

#[test]
fn test_type_723() {
    let result = parse_type("struct /*VFT*/ CGameObjectCreator<CSungunWeapon>_vtbl");
    assert_eq!(result, vec!["CGameObjectCreator<CSungunWeapon>_vtbl"]);
}

#[test]
fn test_type_724() {
    let result = parse_type("struct /*VFT*/ CGameObjectCreator<CWingsuitRing>_vtbl");
    assert_eq!(result, vec!["CGameObjectCreator<CWingsuitRing>_vtbl"]);
}

#[test]
fn test_type_725() {
    let result = parse_type("struct /*VFT*/ CGraphEntryOccupiedByAuthorityCondition_vtbl");
    assert_eq!(result, vec!["CGraphEntryOccupiedByAuthorityCondition_vtbl"]);
}

#[test]
fn test_type_726() {
    let result = parse_type("struct /*VFT*/ CHealthControl_vtbl");
    assert_eq!(result, vec!["CHealthControl_vtbl"]);
}

#[test]
fn test_type_727() {
    let result = parse_type("struct /*VFT*/ CHorizontalSpeedCondition_vtbl");
    assert_eq!(result, vec!["CHorizontalSpeedCondition_vtbl"]);
}

#[test]
fn test_type_728() {
    let result = parse_type("struct /*VFT*/ CMatchmakingProvider_Lan_vtbl");
    assert_eq!(result, vec!["CMatchmakingProvider_Lan_vtbl"]);
}

#[test]
fn test_type_729() {
    let result = parse_type("struct /*VFT*/ COnlineAppSystemFactory_vtbl");
    assert_eq!(result, vec!["COnlineAppSystemFactory_vtbl"]);
}

#[test]
fn test_type_730() {
    let result = parse_type(
        "struct /*VFT*/ COperation_Functor<`anonymous namespace'::<lambda111>,CCallContext>_vtbl",
    );
    assert_eq!(
        result,
        vec!["COperation_Functor<`anonymous namespace'::<lambda111>,CCallContext>_vtbl"]
    );
}

#[test]
fn test_type_731() {
    let result = parse_type(
        "struct /*VFT*/ COperation_Functor<`anonymous namespace'::<lambda40>,CCallContext>_vtbl",
    );
    assert_eq!(
        result,
        vec!["COperation_Functor<`anonymous namespace'::<lambda40>,CCallContext>_vtbl"]
    );
}

#[test]
fn test_type_732() {
    let result = parse_type("struct /*VFT*/ CPlantedExplosiveWeapon_vtbl");
    assert_eq!(result, vec!["CPlantedExplosiveWeapon_vtbl"]);
}

#[test]
fn test_type_733() {
    let result = parse_type("struct /*VFT*/ CPlaybackManager_vtbl");
    assert_eq!(result, vec!["CPlaybackManager_vtbl"]);
}

#[test]
fn test_type_734() {
    let result = parse_type("struct /*VFT*/ CScreenshotCommand_vtbl");
    assert_eq!(result, vec!["CScreenshotCommand_vtbl"]);
}

#[test]
fn test_type_735() {
    let result = parse_type("struct /*VFT*/ CTransitionNode_vtbl");
    assert_eq!(result, vec!["CTransitionNode_vtbl"]);
}

#[test]
fn test_type_736() {
    let result = parse_type("struct /*VFT*/ hkaiDefaultTask_vtbl");
    assert_eq!(result, vec!["hkaiDefaultTask_vtbl"]);
}

#[test]
fn test_type_737() {
    let result = parse_type("struct /*VFT*/ hkaiSilhouetteRecorder::GraphUnloadedEvent_vtbl");
    assert_eq!(
        result,
        vec!["hkaiSilhouetteRecorder", "GraphUnloadedEvent_vtbl"]
    );
}

#[test]
fn test_type_738() {
    let result = parse_type("struct /*VFT*/ hkaiUserEdgePairArray_vtbl");
    assert_eq!(result, vec!["hkaiUserEdgePairArray_vtbl"]);
}

#[test]
fn test_type_739() {
    let result = parse_type("struct /*VFT*/ hkcdConvexCellsCollection_vtbl");
    assert_eq!(result, vec!["hkcdConvexCellsCollection_vtbl"]);
}

#[test]
fn test_type_740() {
    let result = parse_type("struct /*VFT*/ hkcdConvexCellsTree2D_vtbl");
    assert_eq!(result, vec!["hkcdConvexCellsTree2D_vtbl"]);
}

#[test]
fn test_type_741() {
    let result = parse_type("struct /*VFT*/ hkcdSimdTree::PairCollector_vtbl");
    assert_eq!(result, vec!["hkcdSimdTree", "PairCollector_vtbl"]);
}

#[test]
fn test_type_742() {
    let result =
        parse_type("struct /*VFT*/ hkCrcStreamWriter<unsigned __int64,-3932672073523589310>_vtbl");
    assert_eq!(
        result,
        vec!["hkCrcStreamWriter<unsigned __int64,-3932672073523589310>_vtbl"]
    );
}

#[test]
fn test_type_743() {
    let result = parse_type("struct /*VFT*/ hkdMeshGraphicsBody_vtbl");
    assert_eq!(result, vec!["hkdMeshGraphicsBody_vtbl"]);
}

#[test]
fn test_type_744() {
    let result = parse_type("struct /*VFT*/ hkdWoodFracture_vtbl");
    assert_eq!(result, vec!["hkdWoodFracture_vtbl"]);
}

#[test]
fn test_type_745() {
    let result = parse_type("struct /*VFT*/ hkndBodyTimeoutRuntime_vtbl");
    assert_eq!(result, vec!["hkndBodyTimeoutRuntime_vtbl"]);
}

#[test]
fn test_type_746() {
    let result = parse_type("struct /*VFT*/ hkndMissileGun_vtbl");
    assert_eq!(result, vec!["hkndMissileGun_vtbl"]);
}

#[test]
fn test_type_747() {
    let result = parse_type("struct /*VFT*/ hknpManifoldViewer_vtbl");
    assert_eq!(result, vec!["hknpManifoldViewer_vtbl"]);
}

#[test]
fn test_type_748() {
    let result = parse_type("struct /*VFT*/ hkpCollisionFilter_vtbl");
    assert_eq!(result, vec!["hkpCollisionFilter_vtbl"]);
}

#[test]
fn test_type_749() {
    let result = parse_type("struct /*VFT*/ hkpVehicleCastBatchingManager_vtbl");
    assert_eq!(result, vec!["hkpVehicleCastBatchingManager_vtbl"]);
}

#[test]
fn test_type_750() {
    let result = parse_type(
        "struct /*VFT*/ hkSignal2<hknpWorld *,hkHandle<unsigned int,2147483647,hknpConstraintIdDiscriminant> >::Slot_vtbl",
    );
    assert_eq!(
        result,
        vec![
            "hkSignal2<hknpWorld *,hkHandle<unsigned int,2147483647,hknpConstraintIdDiscriminant> >",
            "Slot_vtbl"
        ]
    );
}

#[test]
fn test_type_751() {
    let result = parse_type("struct /*VFT*/ hkSignal2<hknpWorld *,hknpConstraint *>::Slot_vtbl");
    assert_eq!(
        result,
        vec!["hkSignal2<hknpWorld *,hknpConstraint *>", "Slot_vtbl"]
    );
}

#[test]
fn test_type_752() {
    let result = parse_type(
        "struct /*VFT*/ hkSignal3<hkndWorld *,hkHandle<unsigned short,65535,hkndBodyUniqueId>,hkArray<hkndDestructionApiCommandProcessor::BodyReplacementInfo,hkContainerHeapAllocator> const &>::MemberSlot<hkndFlexibleJointRuntime,void (__cdecl hkndFlexibleJointRuntime::*)(hkndWorld *,hkHandle<unsigned short,65535,hkndBodyUniqueId>,hkArray<hkndDestructionApiCommandProcessor::BodyReplacementInfo,hkContainerHeapAllocator> const &)>_vtbl",
    );
    assert_eq!(
        result,
        vec![
            "hkSignal3<hkndWorld *,hkHandle<unsigned short,65535,hkndBodyUniqueId>,hkArray<hkndDestructionApiCommandProcessor::BodyReplacementInfo,hkContainerHeapAllocator> const &>",
            "MemberSlot<hkndFlexibleJointRuntime,void (__cdecl hkndFlexibleJointRuntime::*)(hkndWorld *,hkHandle<unsigned short,65535,hkndBodyUniqueId>,hkArray<hkndDestructionApiCommandProcessor::BodyReplacementInfo,hkContainerHeapAllocator> const &)>_vtbl"
        ]
    );
}

#[test]
fn test_type_753() {
    let result = parse_type(
        "struct /*VFT*/ hkSignal3<hknpWorld *,hknpBodyId,bool>::MemberSlot<hknpCollisionSkinViewer,void (__cdecl hknpCollisionSkinViewer::*)(hknpWorld *,hknpBodyId,bool)>_vtbl",
    );
    assert_eq!(
        result,
        vec![
            "hkSignal3<hknpWorld *,hknpBodyId,bool>",
            "MemberSlot<hknpCollisionSkinViewer,void (__cdecl hknpCollisionSkinViewer::*)(hknpWorld *,hknpBodyId,bool)>_vtbl"
        ]
    );
}

#[test]
fn test_type_754() {
    let result = parse_type(
        "struct /*VFT*/ hkSignal3<hknpWorld *,hknpBodyId,hkRefPtr<hknpShape const > &>::MemberSlot<CShapeTagCodec,void (__cdecl CShapeTagCodec::*)(hknpWorld *,hknpBodyId,hkRefPtr<hknpShape const > &)const >_vtbl",
    );
    assert_eq!(
        result,
        vec![
            "hkSignal3<hknpWorld *,hknpBodyId,hkRefPtr<hknpShape const > &>",
            "MemberSlot<CShapeTagCodec,void (__cdecl CShapeTagCodec::*)(hknpWorld *,hknpBodyId,hkRefPtr<hknpShape const > &)const >_vtbl"
        ]
    );
}

#[test]
fn test_type_755() {
    let result = parse_type("struct /*VFT*/ ID2D1SimplifiedGeometrySink_vtbl");
    assert_eq!(result, vec!["ID2D1SimplifiedGeometrySink_vtbl"]);
}

#[test]
fn test_type_756() {
    let result = parse_type("struct /*VFT*/ ISpawnHelper_vtbl");
    assert_eq!(result, vec!["ISpawnHelper_vtbl"]);
}

#[test]
fn test_type_757() {
    let result = parse_type(
        "struct /*VFT*/ NAnimationSystem::CSpecificCreator<NAnimationSystem::CBoxBase,NAnimationSystem::CAddRotationsBox>_vtbl",
    );
    assert_eq!(
        result,
        vec![
            "NAnimationSystem",
            "CSpecificCreator<NAnimationSystem::CBoxBase,NAnimationSystem::CAddRotationsBox>_vtbl"
        ]
    );
}

#[test]
fn test_type_758() {
    let result = parse_type("struct /*VFT*/ NAnimationSystem::CVectorToNumberBox_vtbl");
    assert_eq!(result, vec!["NAnimationSystem", "CVectorToNumberBox_vtbl"]);
}

#[test]
fn test_type_759() {
    let result =
        parse_type("struct /*VFT*/ NGraphicsEngine::CRenderBlockBark::CRenderBlockBarkType_vtbl");
    assert_eq!(
        result,
        vec![
            "NGraphicsEngine",
            "CRenderBlockBark",
            "CRenderBlockBarkType_vtbl"
        ]
    );
}

#[test]
fn test_type_760() {
    let result = parse_type(
        "struct /*VFT*/ NGraphicsEngine::CRenderBlockRainOccluder::CRenderBlockTypeRainOccluder_vtbl",
    );
    assert_eq!(
        result,
        vec![
            "NGraphicsEngine",
            "CRenderBlockRainOccluder",
            "CRenderBlockTypeRainOccluder_vtbl"
        ]
    );
}

#[test]
fn test_type_761() {
    let result =
        parse_type("struct /*VFT*/ NGraphicsEngine::CTerrainRenderBlockDetail::CDetailRBType_vtbl");
    assert_eq!(
        result,
        vec![
            "NGraphicsEngine",
            "CTerrainRenderBlockDetail",
            "CDetailRBType_vtbl"
        ]
    );
}

#[test]
fn test_type_762() {
    let result = parse_type("struct /*VFT*/ NSaveLoad::CRemoteDataRef<int>_vtbl");
    assert_eq!(result, vec!["NSaveLoad", "CRemoteDataRef<int>_vtbl"]);
}

#[test]
fn test_type_763() {
    let result = parse_type("struct /*VFT*/ NUIManager::ScaleFormText_vtbl");
    assert_eq!(result, vec!["NUIManager", "ScaleFormText_vtbl"]);
}

#[test]
fn test_type_764() {
    let result = parse_type(
        "struct /*VFT*/ OSuite::TConstIterator<OSuite::TOrderedMap<OSuite::ZString,OSuite::ZMetric::IStoredData *,OSuite::TOperatorComparer<OSuite::ZString> >::ZIterator,OSuite::ZMetric::IStoredData *,OSuite::ZString>_vtbl",
    );
    assert_eq!(
        result,
        vec![
            "OSuite",
            "TConstIterator<OSuite::TOrderedMap<OSuite::ZString,OSuite::ZMetric::IStoredData *,OSuite::TOperatorComparer<OSuite::ZString> >::ZIterator,OSuite::ZMetric::IStoredData *,OSuite::ZString>_vtbl"
        ]
    );
}

#[test]
fn test_type_765() {
    let result = parse_type("struct /*VFT*/ OSuite::TList<OSuite::ZOEdmEntitySet *>_vtbl");
    assert_eq!(
        result,
        vec!["OSuite", "TList<OSuite::ZOEdmEntitySet *>_vtbl"]
    );
}

#[test]
fn test_type_766() {
    let result =
        parse_type("struct /*VFT*/ OSuite::TMap<OSuite::ZString,enum OSuite::ZHttp::EError>_vtbl");
    assert_eq!(
        result,
        vec![
            "OSuite",
            "TMap<OSuite::ZString,enum OSuite::ZHttp::EError>_vtbl"
        ]
    );
}

#[test]
fn test_type_767() {
    let result = parse_type(
        "struct /*VFT*/ OSuite::TOrderedMap<OSuite::ZString,enum OSuite::ZHttp::EError,OSuite::TOperatorComparer<OSuite::ZString> >::ZIterator_vtbl",
    );
    assert_eq!(
        result,
        vec![
            "OSuite",
            "TOrderedMap<OSuite::ZString,enum OSuite::ZHttp::EError,OSuite::TOperatorComparer<OSuite::ZString> >",
            "ZIterator_vtbl"
        ]
    );
}

#[test]
fn test_type_768() {
    let result = parse_type(
        "struct /*VFT*/ OSuite::TPair<int,OSuitePrivate::FacebookClientDelegate *>_vtbl",
    );
    assert_eq!(
        result,
        vec![
            "OSuite",
            "TPair<int,OSuitePrivate::FacebookClientDelegate *>_vtbl"
        ]
    );
}

#[test]
fn test_type_769() {
    let result = parse_type(
        "struct /*VFT*/ OSuite::ZRedBlackTreeBase::TIterator<OSuite::TKeyValueElement<OSuite::ZString,enum OSuite::ZHttp::EError> >_vtbl",
    );
    assert_eq!(
        result,
        vec![
            "OSuite",
            "ZRedBlackTreeBase",
            "TIterator<OSuite::TKeyValueElement<OSuite::ZString,enum OSuite::ZHttp::EError> >_vtbl"
        ]
    );
}

#[test]
fn test_type_770() {
    let result = parse_type("struct /*VFT*/ Scaleform::GFx::AS2::ActionBuffer_vtbl");
    assert_eq!(result, vec!["Scaleform", "GFx", "AS2", "ActionBuffer_vtbl"]);
}

#[test]
fn test_type_771() {
    let result = parse_type(
        "struct /*VFT*/ Scaleform::GFx::AS3::Classes::fl_display::ActionScriptVersion_vtbl",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "GFx",
            "AS3",
            "Classes",
            "fl_display",
            "ActionScriptVersion_vtbl"
        ]
    );
}

#[test]
fn test_type_772() {
    let result =
        parse_type("struct /*VFT*/ Scaleform::GFx::AS3::Classes::fl_gfx::MouseEventEx_vtbl");
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "GFx",
            "AS3",
            "Classes",
            "fl_gfx",
            "MouseEventEx_vtbl"
        ]
    );
}

#[test]
fn test_type_773() {
    let result =
        parse_type("struct /*VFT*/ Scaleform::GFx::AS3::ClassTraits::fl_display::ShaderJob_vtbl");
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "GFx",
            "AS3",
            "ClassTraits",
            "fl_display",
            "ShaderJob_vtbl"
        ]
    );
}

#[test]
fn test_type_774() {
    let result = parse_type(
        "struct /*VFT*/ Scaleform::GFx::AS3::ClassTraits::fl_events::DRMAuthenticateEvent_vtbl",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "GFx",
            "AS3",
            "ClassTraits",
            "fl_events",
            "DRMAuthenticateEvent_vtbl"
        ]
    );
}

#[test]
fn test_type_775() {
    let result = parse_type("struct /*VFT*/ Scaleform::GFx::AS3::ClassTraits::fl_net::Socket_vtbl");
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "GFx",
            "AS3",
            "ClassTraits",
            "fl_net",
            "Socket_vtbl"
        ]
    );
}

#[test]
fn test_type_776() {
    let result =
        parse_type("struct /*VFT*/ Scaleform::GFx::AS3::ClassTraits::fl_utils::IDataInput_vtbl");
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "GFx",
            "AS3",
            "ClassTraits",
            "fl_utils",
            "IDataInput_vtbl"
        ]
    );
}

#[test]
fn test_type_777() {
    let result = parse_type("struct /*VFT*/ Scaleform::GFx::AS3::ClassTraits::fl::RegExp_vtbl");
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "GFx",
            "AS3",
            "ClassTraits",
            "fl",
            "RegExp_vtbl"
        ]
    );
}

#[test]
fn test_type_778() {
    let result = parse_type("struct /*VFT*/ Scaleform::GFx::AS3::IMEManager_vtbl");
    assert_eq!(result, vec!["Scaleform", "GFx", "AS3", "IMEManager_vtbl"]);
}

#[test]
fn test_type_779() {
    let result = parse_type(
        "struct /*VFT*/ Scaleform::GFx::AS3::Instances::fl_events::HTMLUncaughtScriptExceptionEvent_vtbl",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "GFx",
            "AS3",
            "Instances",
            "fl_events",
            "HTMLUncaughtScriptExceptionEvent_vtbl"
        ]
    );
}

#[test]
fn test_type_780() {
    let result =
        parse_type("struct /*VFT*/ Scaleform::GFx::AS3::Instances::fl_ui::ContextMenu_vtbl");
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "GFx",
            "AS3",
            "Instances",
            "fl_ui",
            "ContextMenu_vtbl"
        ]
    );
}

#[test]
fn test_type_781() {
    let result = parse_type(
        "struct /*VFT*/ Scaleform::GFx::AS3::InstanceTraits::fl_desktop::NativeDragOptions_vtbl",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "GFx",
            "AS3",
            "InstanceTraits",
            "fl_desktop",
            "NativeDragOptions_vtbl"
        ]
    );
}

#[test]
fn test_type_782() {
    let result =
        parse_type("struct /*VFT*/ Scaleform::GFx::AS3::InstanceTraits::fl_utils::Timer_vtbl");
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "GFx",
            "AS3",
            "InstanceTraits",
            "fl_utils",
            "Timer_vtbl"
        ]
    );
}

#[test]
fn test_type_783() {
    let result = parse_type("struct /*VFT*/ Scaleform::GFx::AS3::MovieRoot::StickyVarNode_vtbl");
    assert_eq!(
        result,
        vec!["Scaleform", "GFx", "AS3", "MovieRoot", "StickyVarNode_vtbl"]
    );
}

#[test]
fn test_type_784() {
    let result = parse_type("struct /*VFT*/ Scaleform::GFx::AS3Support_vtbl");
    assert_eq!(result, vec!["Scaleform", "GFx", "AS3Support_vtbl"]);
}

#[test]
fn test_type_785() {
    let result = parse_type("struct /*VFT*/ Scaleform::GFx::GFxSocketImpl_vtbl");
    assert_eq!(result, vec!["Scaleform", "GFx", "GFxSocketImpl_vtbl"]);
}

#[test]
fn test_type_786() {
    let result = parse_type(
        "struct /*VFT*/ Scaleform::RefCountBase<Scaleform::GFx::AMP::FunctionDesc,578>_vtbl",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "RefCountBase<Scaleform::GFx::AMP::FunctionDesc,578>_vtbl"
        ]
    );
}

#[test]
fn test_type_787() {
    let result = parse_type(
        "struct /*VFT*/ Scaleform::RefCountBaseStatImpl<Scaleform::RefCountImpl,579>_vtbl",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "RefCountBaseStatImpl<Scaleform::RefCountImpl,579>_vtbl"
        ]
    );
}

#[test]
fn test_type_788() {
    let result = parse_type(
        "struct /*VFT*/ Scaleform::RefCountBaseStatImpl<Scaleform::RefCountNTSImpl,4>_vtbl",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "RefCountBaseStatImpl<Scaleform::RefCountNTSImpl,4>_vtbl"
        ]
    );
}

#[test]
fn test_type_789() {
    let result = parse_type("struct /*VFT*/ Scaleform::Render::ContextImpl::RenderNotify_vtbl");
    assert_eq!(
        result,
        vec!["Scaleform", "Render", "ContextImpl", "RenderNotify_vtbl"]
    );
}

#[test]
fn test_type_790() {
    let result = parse_type("struct /*VFT*/ Scaleform::Render::GlyphCache_vtbl");
    assert_eq!(result, vec!["Scaleform", "Render", "GlyphCache_vtbl"]);
}

#[test]
fn test_type_791() {
    let result = parse_type(
        "struct /*VFT*/ Scaleform::Render::ImageFileWriter_Mixin<Scaleform::Render::SIF::FileWriter,Scaleform::Render::ImageFileWriter>_vtbl",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "Render",
            "ImageFileWriter_Mixin<Scaleform::Render::SIF::FileWriter,Scaleform::Render::ImageFileWriter>_vtbl"
        ]
    );
}

#[test]
fn test_type_792() {
    let result = parse_type("struct /*VFT*/ Scaleform::Render::ProfileModifierTDensity_vtbl");
    assert_eq!(
        result,
        vec!["Scaleform", "Render", "ProfileModifierTDensity_vtbl"]
    );
}

#[test]
fn test_type_793() {
    let result = parse_type("struct /*VFT*/ Scaleform::Render::Text::ImageDesc_vtbl");
    assert_eq!(
        result,
        vec!["Scaleform", "Render", "Text", "ImageDesc_vtbl"]
    );
}

#[test]
fn test_type_794() {
    let result = parse_type("struct /*VFT*/ Scaleform::Render::TGA::FileWriter_vtbl");
    assert_eq!(
        result,
        vec!["Scaleform", "Render", "TGA", "FileWriter_vtbl"]
    );
}

#[test]
fn test_type_795() {
    let result = parse_type("struct /*VFT*/ Scaleform::ResouceProvider_vtbl");
    assert_eq!(result, vec!["Scaleform", "ResouceProvider_vtbl"]);
}

#[test]
fn test_type_796() {
    let result = parse_type("struct /*VFT*/ std::runtime_error_vtbl");
    assert_eq!(result, vec!["std", "runtime_error_vtbl"]);
}

#[test]
fn test_type_797() {
    let result = parse_type(
        "struct /*VFT*/ std::tr1::_Impl_no_alloc0<std::tr1::_Callable_obj<`anonymous namespace'::<lambda100>,0>,void>_vtbl",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "tr1",
            "_Impl_no_alloc0<std::tr1::_Callable_obj<`anonymous namespace'::<lambda100>,0>,void>_vtbl"
        ]
    );
}

#[test]
fn test_type_798() {
    let result = parse_type(
        "struct /*VFT*/ std::tr1::_Impl_no_alloc0<std::tr1::_Callable_obj<NPfxBreakable::SDeferredAngularImpulseApplicator_World<CPfxBreakable,1259190699>,0>,void>_vtbl",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "tr1",
            "_Impl_no_alloc0<std::tr1::_Callable_obj<NPfxBreakable::SDeferredAngularImpulseApplicator_World<CPfxBreakable,1259190699>,0>,void>_vtbl"
        ]
    );
}

#[test]
fn test_type_799() {
    let result = parse_type(
        "struct /*VFT*/ std::tr1::_Impl_no_alloc1<std::tr1::_Callable_obj<`anonymous namespace'::<lambda100>,0>,void,CCallContextResult<boost::shared_ptr<CFriend const > > &>_vtbl",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "tr1",
            "_Impl_no_alloc1<std::tr1::_Callable_obj<`anonymous namespace'::<lambda100>,0>,void,CCallContextResult<boost::shared_ptr<CFriend const > > &>_vtbl"
        ]
    );
}

#[test]
fn test_type_800() {
    let result = parse_type(
        "struct /*VFT*/ std::tr1::_Impl_no_alloc1<std::tr1::_Callable_obj<`anonymous namespace'::<lambda14>,0>,void,COnlineQueryResult const &>_vtbl",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "tr1",
            "_Impl_no_alloc1<std::tr1::_Callable_obj<`anonymous namespace'::<lambda14>,0>,void,COnlineQueryResult const &>_vtbl"
        ]
    );
}

#[test]
fn test_type_801() {
    let result = parse_type(
        "struct /*VFT*/ std::tr1::_Impl_no_alloc1<std::tr1::_Callable_obj<`anonymous namespace'::<lambda182>,0>,void,CCallContextResult<STaskAttemptResult> &>_vtbl",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "tr1",
            "_Impl_no_alloc1<std::tr1::_Callable_obj<`anonymous namespace'::<lambda182>,0>,void,CCallContextResult<STaskAttemptResult> &>_vtbl"
        ]
    );
}

#[test]
fn test_type_802() {
    let result = parse_type(
        "struct /*VFT*/ std::tr1::_Impl_no_alloc1<std::tr1::_Callable_obj<`anonymous namespace'::<lambda69>,0>,void,float const *>_vtbl",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "tr1",
            "_Impl_no_alloc1<std::tr1::_Callable_obj<`anonymous namespace'::<lambda69>,0>,void,float const *>_vtbl"
        ]
    );
}

#[test]
fn test_type_803() {
    let result = parse_type(
        "struct /*VFT*/ std::tr1::_Impl_no_alloc2<std::tr1::_Callable_obj<`anonymous namespace'::<lambda52>,0>,void,CLeaderboardPage<SLeaderboardEntry<__int64>,__int64> &,enum NOnline::EErrorCode>_vtbl",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "tr1",
            "_Impl_no_alloc2<std::tr1::_Callable_obj<`anonymous namespace'::<lambda52>,0>,void,CLeaderboardPage<SLeaderboardEntry<__int64>,__int64> &,enum NOnline::EErrorCode>_vtbl"
        ]
    );
}

#[test]
fn test_type_804() {
    let result = parse_type("struct /*VFT*/ std::wostream_vtbl");
    assert_eq!(result, vec!["std", "wostream_vtbl"]);
}

#[test]
fn test_type_805() {
    let result = parse_type("struct /*VFT*/ std::wstreambuf_vtbl");
    assert_eq!(result, vec!["std", "wstreambuf_vtbl"]);
}

#[test]
fn test_type_806() {
    let result = parse_type("struct /*VFT*/ TAdfStructPtr<SDriverLean>_vtbl");
    assert_eq!(result, vec!["TAdfStructPtr<SDriverLean>_vtbl"]);
}

#[test]
fn test_type_807() {
    let result = parse_type("struct /*VFT*/ TaskQueue::detail::CExecutor<void,void>_vtbl");
    assert_eq!(
        result,
        vec!["TaskQueue", "detail", "CExecutor<void,void>_vtbl"]
    );
}

#[test]
fn test_type_808() {
    let result = parse_type("struct /*VFT*/ testing::ScopedFakeTestPartResultReporter_vtbl");
    assert_eq!(
        result,
        vec!["testing", "ScopedFakeTestPartResultReporter_vtbl"]
    );
}

#[test]
fn test_type_809() {
    let result = parse_type("struct $_TypeDescriptor$_extraBytes_384");
    assert_eq!(result, vec!["$_TypeDescriptor$_extraBytes_384"]);
}

#[test]
fn test_type_810() {
    let result = parse_type("struct $_TypeDescriptor$_extraBytes_40");
    assert_eq!(result, vec!["$_TypeDescriptor$_extraBytes_40"]);
}

#[test]
fn test_type_811() {
    let result = parse_type("struct $B0414F1DEC363B3EFC1FC6E4B7F77361");
    assert_eq!(result, vec!["$B0414F1DEC363B3EFC1FC6E4B7F77361"]);
}

#[test]
fn test_type_812() {
    let result = parse_type("struct $CABC8D72D33B996952E9BFEE2726C6EC");
    assert_eq!(result, vec!["$CABC8D72D33B996952E9BFEE2726C6EC"]);
}

#[test]
fn test_type_813() {
    let result = parse_type("struct $CB9BE7F75001A8B17FD6A7B54E1255C4");
    assert_eq!(result, vec!["$CB9BE7F75001A8B17FD6A7B54E1255C4"]);
}

#[test]
fn test_type_814() {
    let result = parse_type(
        "struct boost::detail::function::basic_vtable4<bool,char const * &,char const * const &,boost::spirit::context<boost::fusion::cons<dynamo::ast::intrinsic_op &,boost::fusion::nil_>,boost::fusion::vector0<void> > &,boost::spirit::qi::char_class<boost::spirit::tag::char_code<boost::spirit::tag::space,boost::spirit::char_encoding::ascii> > const &>",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "detail",
            "function",
            "basic_vtable4<bool,char const * &,char const * const &,boost::spirit::context<boost::fusion::cons<dynamo::ast::intrinsic_op &,boost::fusion::nil_>,boost::fusion::vector0<void> > &,boost::spirit::qi::char_class<boost::spirit::tag::char_code<boost::spirit::tag::space,boost::spirit::char_encoding::ascii> > const &>"
        ]
    );
}

#[test]
fn test_type_815() {
    let result = parse_type(
        "struct boost::phoenix::evaluator::impl<boost::proto::exprns_::basic_expr<boost::phoenix::detail::tag::function_eval,boost::proto::argsns_::list3<boost::proto::exprns_::basic_expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<dynamo::annotation<char const *> >,0>,boost::phoenix::actor<boost::spirit::attribute<0> >,boost::phoenix::actor<boost::spirit::argument<2> > >,3> const &,boost::phoenix::vector2<boost::mpl::bool_<1>,boost::phoenix::is_nullary>,boost::proto::envns_::empty_env>",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "phoenix",
            "evaluator",
            "impl<boost::proto::exprns_::basic_expr<boost::phoenix::detail::tag::function_eval,boost::proto::argsns_::list3<boost::proto::exprns_::basic_expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<dynamo::annotation<char const *> >,0>,boost::phoenix::actor<boost::spirit::attribute<0> >,boost::phoenix::actor<boost::spirit::argument<2> > >,3> const &,boost::phoenix::vector2<boost::mpl::bool_<1>,boost::phoenix::is_nullary>,boost::proto::envns_::empty_env>"
        ]
    );
}

#[test]
fn test_type_816() {
    let result = parse_type(
        "struct boost::proto::reverse_fold<boost::proto::_,boost::proto::make<boost::fusion::nil_>,boost::proto::detail::reverse_fold_tree_<boost::proto::tagns_::tag::shift_right,boost::spirit::detail::make_binary_helper<boost::spirit::meta_compiler<boost::spirit::qi::domain>::meta_grammar> > >::impl<boost::proto::exprns_::expr<boost::proto::tagns_::tag::shift_right,boost::proto::argsns_::list2<boost::proto::exprns_::expr<boost::proto::tagns_::tag::logical_not,boost::proto::argsns_::list1<boost::proto::exprns_::expr<boost::proto::tagns_::tag::subscript,boost::proto::argsns_::list2<boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::lexeme>,0> &,boost::proto::exprns_::expr<boost::proto::tagns_::tag::shift_right,boost::proto::argsns_::list2<boost::spirit::qi::symbols<char,enum dynamo::ast::op_token,boost::spirit::qi::tst<char,enum dynamo::ast::op_token>,boost::spirit::qi::tst_pass_through> &,boost::proto::exprns_::expr<boost::proto::tagns_::tag::logical_not,boost::proto::argsns_::list1<boost::proto::exprns_::expr<boost::proto::tagns_::tag::bitwise_or,boost::proto::argsns_::list2<boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::char_code<boost::spirit::tag::alnum,boost::spirit::char_encoding::standard> >,0> &,boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<char const &>,0> >,2> const &>,1> const &>,2> const &>,2> const &>,1> const &,boost::proto::exprns_::expr<boost::proto::tagns_::tag::subscript,boost::proto::argsns_::list2<boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::raw>,0> &,boost::proto::exprns_::expr<boost::proto::tagns_::tag::subscript,boost::proto::argsns_::list2<boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::lexeme>,0> &,boost::proto::exprns_::expr<boost::proto::tagns_::tag::shift_right,boost::proto::argsns_::list2<boost::proto::exprns_::expr<boost::proto::tagns_::tag::bitwise_or,boost::proto::argsns_::list2<boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::char_code<boost::spirit::tag::alpha,boost::spirit::char_encoding::standard> >,0> &,boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<char const &>,0> >,2> const &,boost::proto::exprns_::expr<boost::proto::tagns_::tag::dereference,boost::proto::argsns_::list1<boost::proto::exprns_::expr<boost::proto::tagns_::tag::bitwise_or,boost::proto::argsns_::list2<boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::char_code<boost::spirit::tag::alnum,boost::spirit::char_encoding::standard> >,0> &,boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<char const &>,0> >,2> const &>,1> const &>,2> const &>,2> const &>,2> const &>,2>,boost::mpl::void_,boost::spirit::unused_type>",
    );
    assert_eq!(
        result,
        vec![
            "boost",
            "proto",
            "reverse_fold<boost::proto::_,boost::proto::make<boost::fusion::nil_>,boost::proto::detail::reverse_fold_tree_<boost::proto::tagns_::tag::shift_right,boost::spirit::detail::make_binary_helper<boost::spirit::meta_compiler<boost::spirit::qi::domain>::meta_grammar> > >",
            "impl<boost::proto::exprns_::expr<boost::proto::tagns_::tag::shift_right,boost::proto::argsns_::list2<boost::proto::exprns_::expr<boost::proto::tagns_::tag::logical_not,boost::proto::argsns_::list1<boost::proto::exprns_::expr<boost::proto::tagns_::tag::subscript,boost::proto::argsns_::list2<boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::lexeme>,0> &,boost::proto::exprns_::expr<boost::proto::tagns_::tag::shift_right,boost::proto::argsns_::list2<boost::spirit::qi::symbols<char,enum dynamo::ast::op_token,boost::spirit::qi::tst<char,enum dynamo::ast::op_token>,boost::spirit::qi::tst_pass_through> &,boost::proto::exprns_::expr<boost::proto::tagns_::tag::logical_not,boost::proto::argsns_::list1<boost::proto::exprns_::expr<boost::proto::tagns_::tag::bitwise_or,boost::proto::argsns_::list2<boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::char_code<boost::spirit::tag::alnum,boost::spirit::char_encoding::standard> >,0> &,boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<char const &>,0> >,2> const &>,1> const &>,2> const &>,2> const &>,1> const &,boost::proto::exprns_::expr<boost::proto::tagns_::tag::subscript,boost::proto::argsns_::list2<boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::raw>,0> &,boost::proto::exprns_::expr<boost::proto::tagns_::tag::subscript,boost::proto::argsns_::list2<boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::lexeme>,0> &,boost::proto::exprns_::expr<boost::proto::tagns_::tag::shift_right,boost::proto::argsns_::list2<boost::proto::exprns_::expr<boost::proto::tagns_::tag::bitwise_or,boost::proto::argsns_::list2<boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::char_code<boost::spirit::tag::alpha,boost::spirit::char_encoding::standard> >,0> &,boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<char const &>,0> >,2> const &,boost::proto::exprns_::expr<boost::proto::tagns_::tag::dereference,boost::proto::argsns_::list1<boost::proto::exprns_::expr<boost::proto::tagns_::tag::bitwise_or,boost::proto::argsns_::list2<boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::char_code<boost::spirit::tag::alnum,boost::spirit::char_encoding::standard> >,0> &,boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<char const &>,0> >,2> const &>,1> const &>,2> const &>,2> const &>,2> const &>,2>,boost::mpl::void_,boost::spirit::unused_type>"
        ]
    );
}

#[test]
fn test_type_817() {
    let result = parse_type("struct D2D_POINT_2F");
    assert_eq!(result, vec!["D2D_POINT_2F"]);
}

#[test]
fn test_type_818() {
    let result = parse_type("struct DIDEVICEINSTANCEW");
    assert_eq!(result, vec!["DIDEVICEINSTANCEW"]);
}

#[test]
fn test_type_819() {
    let result = parse_type("struct FMOD_CODEC_WAVEFORMAT");
    assert_eq!(result, vec!["FMOD_CODEC_WAVEFORMAT"]);
}

#[test]
fn test_type_820() {
    let result = parse_type("struct FMOD_VECTOR");
    assert_eq!(result, vec!["FMOD_VECTOR"]);
}

#[test]
fn test_type_821() {
    let result = parse_type("struct Graphics::HVertexProgram_t");
    assert_eq!(result, vec!["Graphics", "HVertexProgram_t"]);
}

#[test]
fn test_type_822() {
    let result = parse_type("struct hkaiAdaptiveRanger_DefaultStruct");
    assert_eq!(result, vec!["hkaiAdaptiveRanger_DefaultStruct"]);
}

#[test]
fn test_type_823() {
    let result = parse_type("struct hkaiNavMeshTraversalUtils::FaceStackNode");
    assert_eq!(result, vec!["hkaiNavMeshTraversalUtils", "FaceStackNode"]);
}

#[test]
fn test_type_824() {
    let result = parse_type("struct hkaiSplitGenerationUtilsSettings_DefaultStruct");
    assert_eq!(
        result,
        vec!["hkaiSplitGenerationUtilsSettings_DefaultStruct"]
    );
}

#[test]
fn test_type_825() {
    let result = parse_type("struct hkGeometryProcessing::SurfaceSampler::Element");
    assert_eq!(
        result,
        vec!["hkGeometryProcessing", "SurfaceSampler", "Element"]
    );
}

#[test]
fn test_type_826() {
    let result = parse_type("struct hkMxMaskd<3>");
    assert_eq!(result, vec!["hkMxMaskd<3>"]);
}

#[test]
fn test_type_827() {
    let result = parse_type(
        "struct hknpCompoundShapeInternals<hknpStaticCompoundShapeInternals>::Accessor<1>",
    );
    assert_eq!(
        result,
        vec![
            "hknpCompoundShapeInternals<hknpStaticCompoundShapeInternals>",
            "Accessor<1>"
        ]
    );
}

#[test]
fn test_type_828() {
    let result = parse_type("struct hkpBuildJacobianTask::AtomInfo");
    assert_eq!(result, vec!["hkpBuildJacobianTask", "AtomInfo"]);
}

#[test]
fn test_type_829() {
    let result = parse_type("struct hkSimpleSchedulerTaskBuilder::TaskInfo");
    assert_eq!(result, vec!["hkSimpleSchedulerTaskBuilder", "TaskInfo"]);
}

#[test]
fn test_type_830() {
    let result = parse_type("struct Input::SDeviceType");
    assert_eq!(result, vec!["Input", "SDeviceType"]);
}

#[test]
fn test_type_831() {
    let result = parse_type("struct jpeg_marker_struct");
    assert_eq!(result, vec!["jpeg_marker_struct"]);
}

#[test]
fn test_type_832() {
    let result = parse_type("struct MemoryBlockStat");
    assert_eq!(result, vec!["MemoryBlockStat"]);
}

#[test]
fn test_type_833() {
    let result = parse_type("struct NDestruction;");
    assert_eq!(result, vec!["NDestruction;"]);
}

#[test]
fn test_type_834() {
    let result = parse_type("struct NGraphicsEngine::SVertexDeformNormal2");
    assert_eq!(result, vec!["NGraphicsEngine", "SVertexDeformNormal2"]);
}

#[test]
fn test_type_835() {
    let result = parse_type("struct ObjectCopier_DummyArray<char>");
    assert_eq!(result, vec!["ObjectCopier_DummyArray<char>"]);
}

#[test]
fn test_type_836() {
    let result = parse_type("struct SBone");
    assert_eq!(result, vec!["SBone"]);
}

#[test]
fn test_type_837() {
    let result = parse_type("struct Scaleform::Render::D3D1x::VertexShaderDesc::VertexAttrDesc");
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "Render",
            "D3D1x",
            "VertexShaderDesc",
            "VertexAttrDesc"
        ]
    );
}

#[test]
fn test_type_838() {
    let result = parse_type("struct SDataLifespans");
    assert_eq!(result, vec!["SDataLifespans"]);
}

#[test]
fn test_type_839() {
    let result = parse_type("struct SEffectAttachment");
    assert_eq!(result, vec!["SEffectAttachment"]);
}

#[test]
fn test_type_840() {
    let result = parse_type("struct SGear");
    assert_eq!(result, vec!["SGear"]);
}

#[test]
fn test_type_841() {
    let result = parse_type("struct SLandSteering");
    assert_eq!(result, vec!["SLandSteering"]);
}

#[test]
fn test_type_842() {
    let result = parse_type("struct SMissileWeaponTuning");
    assert_eq!(result, vec!["SMissileWeaponTuning"]);
}

#[test]
fn test_type_843() {
    let result = parse_type("struct SMotorbikeSteering");
    assert_eq!(result, vec!["SMotorbikeSteering"]);
}

#[test]
fn test_type_844() {
    let result = parse_type("struct SOfferItem");
    assert_eq!(result, vec!["SOfferItem"]);
}

#[test]
fn test_type_845() {
    let result = parse_type("struct SPartPhysicsMappings");
    assert_eq!(result, vec!["SPartPhysicsMappings"]);
}

#[test]
fn test_type_846() {
    let result = parse_type("struct SStreamPatchFileHeader");
    assert_eq!(result, vec!["SStreamPatchFileHeader"]);
}

#[test]
fn test_type_847() {
    let result = parse_type("struct StreamContactSolver::PairVelocities");
    assert_eq!(result, vec!["StreamContactSolver", "PairVelocities"]);
}

#[test]
fn test_type_848() {
    let result = parse_type("struct tagFILTERKEYS");
    assert_eq!(result, vec!["tagFILTERKEYS"]);
}

#[test]
fn test_type_849() {
    let result = parse_type("struct tagSIZE");
    assert_eq!(result, vec!["tagSIZE"]);
}

#[test]
fn test_type_850() {
    let result = parse_type(
        "struct TGrowableArray<SEffectInstanceData::SParam,8,0>::<unnamed_tag>::<unnamed_type_m_Aligner>",
    );
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
fn test_type_851() {
    let result = parse_type("typedef __int16 int16;");
    assert_eq!(result, vec!["int16"]);
}

#[test]
fn test_type_852() {
    let result = parse_type("typedef __int64 SInt64;");
    assert_eq!(result, vec!["SInt64"]);
}

#[test]
fn test_type_853() {
    let result = parse_type("typedef _ADMINISTRATOR_POWER_POLICY ADMINISTRATOR_POWER_POLICY;");
    assert_eq!(result, vec!["ADMINISTRATOR_POWER_POLICY"]);
}

#[test]
fn test_type_854() {
    let result = parse_type("typedef _COMMPROP COMMPROP;");
    assert_eq!(result, vec!["COMMPROP"]);
}

#[test]
fn test_type_855() {
    let result = parse_type("typedef _DIOBJECTDATAFORMAT DIOBJECTDATAFORMAT;");
    assert_eq!(result, vec!["DIOBJECTDATAFORMAT"]);
}

#[test]
fn test_type_856() {
    let result = parse_type("typedef _ENLISTMENT_CRM_INFORMATION *PENLISTMENT_CRM_INFORMATION;");
    assert_eq!(result, vec!["PENLISTMENT_CRM_INFORMATION"]);
}

#[test]
fn test_type_857() {
    let result = parse_type("typedef _EXCEPTION_RECORD *LPEXCEPTION_RECORD;");
    assert_eq!(result, vec!["LPEXCEPTION_RECORD"]);
}

#[test]
fn test_type_858() {
    let result = parse_type("typedef _IMAGE_ROM_OPTIONAL_HEADER IMAGE_ROM_OPTIONAL_HEADER;");
    assert_eq!(result, vec!["IMAGE_ROM_OPTIONAL_HEADER"]);
}

#[test]
fn test_type_859() {
    let result = parse_type("typedef _IMAGEHLP_SYMBOL64 IMAGEHLP_SYMBOL64;");
    assert_eq!(result, vec!["IMAGEHLP_SYMBOL64"]);
}

#[test]
fn test_type_860() {
    let result = parse_type("typedef _OSVERSIONINFOW *POSVERSIONINFO;");
    assert_eq!(result, vec!["POSVERSIONINFO"]);
}

#[test]
fn test_type_861() {
    let result = parse_type("typedef _PROCESSOR_RELATIONSHIP PROCESSOR_RELATIONSHIP;");
    assert_eq!(result, vec!["PROCESSOR_RELATIONSHIP"]);
}

#[test]
fn test_type_862() {
    let result = parse_type("typedef _STARTUPINFOW *LPSTARTUPINFO;");
    assert_eq!(result, vec!["LPSTARTUPINFO"]);
}

#[test]
fn test_type_863() {
    let result = parse_type("typedef _SYSTEM_MANDATORY_LABEL_ACE SYSTEM_MANDATORY_LABEL_ACE;");
    assert_eq!(result, vec!["SYSTEM_MANDATORY_LABEL_ACE"]);
}

#[test]
fn test_type_864() {
    let result = parse_type("typedef _TAPE_WMI_OPERATIONS *PTAPE_WMI_OPERATIONS;");
    assert_eq!(result, vec!["PTAPE_WMI_OPERATIONS"]);
}

#[test]
fn test_type_865() {
    let result = parse_type("typedef _TAPE_WRITE_MARKS *PTAPE_WRITE_MARKS;");
    assert_eq!(result, vec!["PTAPE_WRITE_MARKS"]);
}

#[test]
fn test_type_866() {
    let result = parse_type(
        "typedef _TRANSACTIONMANAGER_LOG_INFORMATION *PTRANSACTIONMANAGER_LOG_INFORMATION;",
    );
    assert_eq!(result, vec!["PTRANSACTIONMANAGER_LOG_INFORMATION"]);
}

#[test]
fn test_type_867() {
    let result = parse_type("typedef _UNIVERSAL_NAME_INFOW *LPUNIVERSAL_NAME_INFOW;");
    assert_eq!(result, vec!["LPUNIVERSAL_NAME_INFOW"]);
}

#[test]
fn test_type_868() {
    let result = parse_type("typedef <lambda10> _A0x743c4eea::<lambda10>;");
    assert_eq!(result, vec!["_A0x743c4eea", "<lambda10>"]);
}

#[test]
fn test_type_869() {
    let result = parse_type("typedef <lambda10> _A0xe40109f6::<lambda10>;");
    assert_eq!(result, vec!["_A0xe40109f6", "<lambda10>"]);
}

#[test]
fn test_type_870() {
    let result = parse_type("typedef <lambda106> _A0x2c6d2d03::<lambda106>;");
    assert_eq!(result, vec!["_A0x2c6d2d03", "<lambda106>"]);
}

#[test]
fn test_type_871() {
    let result = parse_type("typedef <lambda125> _A0x2c6d2d03::<lambda125>;");
    assert_eq!(result, vec!["_A0x2c6d2d03", "<lambda125>"]);
}

#[test]
fn test_type_872() {
    let result = parse_type("typedef <lambda13> _A0x743c4eea::<lambda13>;");
    assert_eq!(result, vec!["_A0x743c4eea", "<lambda13>"]);
}

#[test]
fn test_type_873() {
    let result = parse_type("typedef <lambda15> _A0x0a10f1f7::<lambda15>;");
    assert_eq!(result, vec!["_A0x0a10f1f7", "<lambda15>"]);
}

#[test]
fn test_type_874() {
    let result = parse_type("typedef <lambda155> _A0x743c4eea::<lambda155>;");
    assert_eq!(result, vec!["_A0x743c4eea", "<lambda155>"]);
}

#[test]
fn test_type_875() {
    let result = parse_type("typedef <lambda2> _A0x0d232efb::<lambda2>;");
    assert_eq!(result, vec!["_A0x0d232efb", "<lambda2>"]);
}

#[test]
fn test_type_876() {
    let result = parse_type("typedef <lambda24> _A0xa079f785::<lambda24>;");
    assert_eq!(result, vec!["_A0xa079f785", "<lambda24>"]);
}

#[test]
fn test_type_877() {
    let result = parse_type("typedef <lambda3> _A0xe40109f6::<lambda3>;");
    assert_eq!(result, vec!["_A0xe40109f6", "<lambda3>"]);
}

#[test]
fn test_type_878() {
    let result = parse_type("typedef <lambda38> _A0x743c4eea::<lambda38>;");
    assert_eq!(result, vec!["_A0x743c4eea", "<lambda38>"]);
}

#[test]
fn test_type_879() {
    let result = parse_type("typedef <lambda44> _A0x15f5d329::<lambda44>;");
    assert_eq!(result, vec!["_A0x15f5d329", "<lambda44>"]);
}

#[test]
fn test_type_880() {
    let result = parse_type("typedef <lambda49> _A0x83027457::<lambda49>;");
    assert_eq!(result, vec!["_A0x83027457", "<lambda49>"]);
}

#[test]
fn test_type_881() {
    let result = parse_type("typedef <lambda6> _A0x8fd0df16::<lambda6>;");
    assert_eq!(result, vec!["_A0x8fd0df16", "<lambda6>"]);
}

#[test]
fn test_type_882() {
    let result = parse_type("typedef <lambda61> _A0x83027457::<lambda61>;");
    assert_eq!(result, vec!["_A0x83027457", "<lambda61>"]);
}

#[test]
fn test_type_883() {
    let result = parse_type("typedef <lambda8> _A0x743c4eea::<lambda8>;");
    assert_eq!(result, vec!["_A0x743c4eea", "<lambda8>"]);
}

#[test]
fn test_type_884() {
    let result = parse_type("typedef <lambda8> _A0x78becc47::<lambda8>;");
    assert_eq!(result, vec!["_A0x78becc47", "<lambda8>"]);
}

#[test]
fn test_type_885() {
    let result = parse_type("typedef arith_entropy_decoder *arith_entropy_ptr;");
    assert_eq!(result, vec!["arith_entropy_ptr"]);
}

#[test]
fn test_type_886() {
    let result =
        parse_type("typedef ava::idstring<NAnimationSystem::SModifierIdTag,0> CModifierId;");
    assert_eq!(result, vec!["CModifierId"]);
}

#[test]
fn test_type_887() {
    let result = parse_type("typedef bool (__fastcall *PredicateFunctionType)(hkVariant);");
    assert_eq!(result, vec!["PredicateFunctionType"]);
}

#[test]
fn test_type_888() {
    let result = parse_type(
        "typedef boost::date_time::years_duration<boost::gregorian::greg_durations_config> years;",
    );
    assert_eq!(result, vec!["years"]);
}

#[test]
fn test_type_889() {
    let result = parse_type("typedef CEm *PEM;");
    assert_eq!(result, vec!["PEM"]);
}

#[test]
fn test_type_890() {
    let result = parse_type("typedef const char *PCTSTR;");
    assert_eq!(result, vec!["PCTSTR"]);
}

#[test]
fn test_type_891() {
    let result = parse_type("typedef const png_sPLT_struct *png_const_sPLT_tp;");
    assert_eq!(result, vec!["png_const_sPLT_tp"]);
}

#[test]
fn test_type_892() {
    let result = parse_type("typedef const wchar_t *PCUWSTR;");
    assert_eq!(result, vec!["PCUWSTR"]);
}

#[test]
fn test_type_893() {
    let result = parse_type("typedef DIPROPHEADER *LPDIPROPHEADER;");
    assert_eq!(result, vec!["LPDIPROPHEADER"]);
}

#[test]
fn test_type_894() {
    let result = parse_type(
        "typedef FleeGraph::<unnamed_tag> hkaiHierarchicalGraphHeuristic::<unnamed_tag>;",
    );
    assert_eq!(
        result,
        vec!["hkaiHierarchicalGraphHeuristic", "<unnamed_tag>"]
    );
}

#[test]
fn test_type_895() {
    let result = parse_type("typedef HIMCC__ *HIMCC;");
    assert_eq!(result, vec!["HIMCC"]);
}

#[test]
fn test_type_896() {
    let result = parse_type("typedef hkaiNavVolumeInstance hkaiNavVolumeAccessor;");
    assert_eq!(result, vec!["hkaiNavVolumeAccessor"]);
}

#[test]
fn test_type_897() {
    let result = parse_type(
        "typedef hkaiSilhouetteGenerator::ForceGenerateOntoPpuReasons hkpCollidable::ForceCollideOntoPpuReasons;",
    );
    assert_eq!(result, vec!["hkpCollidable", "ForceCollideOntoPpuReasons"]);
}

#[test]
fn test_type_898() {
    let result = parse_type(
        "typedef hkArrayBase<boost::shared_ptr<CAnimationInstance> >::<unnamed_tag> hkArrayBase<EdgeForEdgeDuplicateCheck>::<unnamed_tag>;",
    );
    assert_eq!(
        result,
        vec!["hkArrayBase<EdgeForEdgeDuplicateCheck>", "<unnamed_tag>"]
    );
}

#[test]
fn test_type_899() {
    let result = parse_type(
        "typedef hkArrayBase<boost::shared_ptr<CAnimationInstance> >::<unnamed_tag> hkArrayBase<hkaSkeleton::LocalFrameOnBone>::<unnamed_tag>;",
    );
    assert_eq!(
        result,
        vec![
            "hkArrayBase<hkaSkeleton::LocalFrameOnBone>",
            "<unnamed_tag>"
        ]
    );
}

#[test]
fn test_type_900() {
    let result = parse_type(
        "typedef hkArrayBase<boost::shared_ptr<CAnimationInstance> >::<unnamed_tag> hkArrayBase<hkBlockStreamBase::LinkedRange>::<unnamed_tag>;",
    );
    assert_eq!(
        result,
        vec![
            "hkArrayBase<hkBlockStreamBase::LinkedRange>",
            "<unnamed_tag>"
        ]
    );
}

#[test]
fn test_type_901() {
    let result = parse_type(
        "typedef hkArrayBase<boost::shared_ptr<CAnimationInstance> >::<unnamed_tag> hkArrayBase<hkgpBoolean::Error>::<unnamed_tag>;",
    );
    assert_eq!(
        result,
        vec!["hkArrayBase<hkgpBoolean::Error>", "<unnamed_tag>"]
    );
}

#[test]
fn test_type_902() {
    let result = parse_type(
        "typedef hkArrayBase<boost::shared_ptr<CAnimationInstance> >::<unnamed_tag> hkArrayBase<hkndDeformationUtil::ConnectionLayer>::<unnamed_tag>;",
    );
    assert_eq!(
        result,
        vec![
            "hkArrayBase<hkndDeformationUtil::ConnectionLayer>",
            "<unnamed_tag>"
        ]
    );
}

#[test]
fn test_type_903() {
    let result = parse_type(
        "typedef hkArrayBase<boost::shared_ptr<CAnimationInstance> >::<unnamed_tag> hkArrayBase<hknpCsContactJacRange const *>::<unnamed_tag>;",
    );
    assert_eq!(
        result,
        vec![
            "hkArrayBase<hknpCsContactJacRange const *>",
            "<unnamed_tag>"
        ]
    );
}

#[test]
fn test_type_904() {
    let result = parse_type(
        "typedef hkArrayBase<boost::shared_ptr<CAnimationInstance> >::<unnamed_tag> hkArrayBase<hknpNarrowPhaseTask::ExistingPairsSubTask>::<unnamed_tag>;",
    );
    assert_eq!(
        result,
        vec![
            "hkArrayBase<hknpNarrowPhaseTask::ExistingPairsSubTask>",
            "<unnamed_tag>"
        ]
    );
}

#[test]
fn test_type_905() {
    let result = parse_type(
        "typedef hkArrayBase<boost::shared_ptr<CAnimationInstance> >::<unnamed_tag> hkArrayBase<hknpVehicleInstance *>::<unnamed_tag>;",
    );
    assert_eq!(
        result,
        vec!["hkArrayBase<hknpVehicleInstance *>", "<unnamed_tag>"]
    );
}

#[test]
fn test_type_906() {
    let result = parse_type(
        "typedef hkArrayBase<boost::shared_ptr<CAnimationInstance> >::<unnamed_tag> hkArrayBase<hkRefPtr<hkMeshShape const > >::<unnamed_tag>;",
    );
    assert_eq!(
        result,
        vec![
            "hkArrayBase<hkRefPtr<hkMeshShape const > >",
            "<unnamed_tag>"
        ]
    );
}

#[test]
fn test_type_907() {
    let result = parse_type(
        "typedef hkArrayBase<boost::shared_ptr<CAnimationInstance> >::<unnamed_tag> hkArrayBase<hkRefPtr<hkxNode> >::<unnamed_tag>;",
    );
    assert_eq!(
        result,
        vec!["hkArrayBase<hkRefPtr<hkxNode> >", "<unnamed_tag>"]
    );
}

#[test]
fn test_type_908() {
    let result = parse_type(
        "typedef hkArrayBase<boost::shared_ptr<CAnimationInstance> >::<unnamed_tag> hkArrayBase<hkResource *>::<unnamed_tag>;",
    );
    assert_eq!(result, vec!["hkArrayBase<hkResource *>", "<unnamed_tag>"]);
}

#[test]
fn test_type_909() {
    let result = parse_type(
        "typedef hkBitFieldBase<hkBitFieldStorage<hkArray<unsigned int,hkContainerHeapAllocator> > > hkBitFieldBasehkBitFieldStoragehkArrayunsignedinthkContainerHeapAllocator_typename;",
    );
    assert_eq!(
        result,
        vec!["hkBitFieldBasehkBitFieldStoragehkArrayunsignedinthkContainerHeapAllocator_typename"]
    );
}

#[test]
fn test_type_910() {
    let result = parse_type(
        "typedef hkBitFieldValue::Uninitialized hkMap<hkDataObject_Handle,int,HandleOps,hkContainerHeapAllocator>::InternalInitializer;",
    );
    assert_eq!(
        result,
        vec![
            "hkMap<hkDataObject_Handle,int,HandleOps,hkContainerHeapAllocator>",
            "InternalInitializer"
        ]
    );
}

#[test]
fn test_type_911() {
    let result = parse_type(
        "typedef hkcdDynamicTree::Tree<hkcdDynamicTree::DynamicStoragePtr> DefaultTreePtr;",
    );
    assert_eq!(result, vec!["DefaultTreePtr"]);
}

#[test]
fn test_type_912() {
    let result = parse_type(
        "typedef hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::UseTwoWayBinary<hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::AllPairsWrapper<hkgpBooleanImpl::NodePair::Collector> >::<unnamed_tag> hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::IfEndFront<hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::AabbOverlapsNearMissAabbWrapper<hknpCompoundShapeInternals<hknpStaticCompoundShapeInternals>::AabbOverlaps<0,hkArray<unsigned int,hkContainerHeapAllocator> > > >::<unnamed_tag>;",
    );
    assert_eq!(
        result,
        vec![
            "hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>",
            "IfEndFront<hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::AabbOverlapsNearMissAabbWrapper<hknpCompoundShapeInternals<hknpStaticCompoundShapeInternals>::AabbOverlaps<0,hkArray<unsigned int,hkContainerHeapAllocator> > > >",
            "<unnamed_tag>"
        ]
    );
}

#[test]
fn test_type_913() {
    let result = parse_type(
        "typedef hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::UseTwoWayBinary<hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::AllPairsWrapper<hkgpBooleanImpl::NodePair::Collector> >::<unnamed_tag> hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::IfEndFront<hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::AabbOverlapsWithEarlyExitWrapper<hkgpMeshInternals::SimpleCollector> >::<unnamed_tag>;",
    );
    assert_eq!(
        result,
        vec![
            "hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>",
            "IfEndFront<hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::AabbOverlapsWithEarlyExitWrapper<hkgpMeshInternals::SimpleCollector> >",
            "<unnamed_tag>"
        ]
    );
}

#[test]
fn test_type_914() {
    let result = parse_type(
        "typedef hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::UseTwoWayBinary<hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::AllPairsWrapper<hkgpBooleanImpl::NodePair::Collector> >::<unnamed_tag> hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::IfGetFront<hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::VisibilityWrapper<hknpCompoundShapeInternals<hknpStaticCompoundShapeInternals>::RayCast<0> > >::<unnamed_tag>;",
    );
    assert_eq!(
        result,
        vec![
            "hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>",
            "IfGetFront<hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::VisibilityWrapper<hknpCompoundShapeInternals<hknpStaticCompoundShapeInternals>::RayCast<0> > >",
            "<unnamed_tag>"
        ]
    );
}

#[test]
fn test_type_915() {
    let result = parse_type(
        "typedef hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::UseTwoWayBinary<hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::AllPairsWrapper<hkgpBooleanImpl::NodePair::Collector> >::<unnamed_tag> hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::IfProcessChildren<hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::AabbOverlapsNearMissAabbWrapper<hknpExternMeshShapeInternals::AabbOverlaps<hkArray<unsigned int,hkContainerHeapAllocator> > >,hkcdStaticTree::DefaultTreeStorage6>::<unnamed_tag>;",
    );
    assert_eq!(
        result,
        vec![
            "hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>",
            "IfProcessChildren<hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::AabbOverlapsNearMissAabbWrapper<hknpExternMeshShapeInternals::AabbOverlaps<hkArray<unsigned int,hkContainerHeapAllocator> > >,hkcdStaticTree::DefaultTreeStorage6>",
            "<unnamed_tag>"
        ]
    );
}

#[test]
fn test_type_916() {
    let result = parse_type(
        "typedef hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::UseTwoWayBinary<hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::AllPairsWrapper<hkgpBooleanImpl::NodePair::Collector> >::<unnamed_tag> hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::IfProcessSelf<hknpCompressedMeshShapeInternals::GetClosestPointsToMeshQueryUnscaled>::<unnamed_tag>;",
    );
    assert_eq!(
        result,
        vec![
            "hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>",
            "IfProcessSelf<hknpCompressedMeshShapeInternals::GetClosestPointsToMeshQueryUnscaled>",
            "<unnamed_tag>"
        ]
    );
}

#[test]
fn test_type_917() {
    let result = parse_type(
        "typedef hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::UseTwoWayBinary<hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::AllPairsWrapper<hkgpBooleanImpl::NodePair::Collector> >::<unnamed_tag> hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::IfPushFront<hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::AabbCastWrapperExternal<hknpCompressedMeshShapeInternals::ShapeCastQueryUnscaled> >::<unnamed_tag>;",
    );
    assert_eq!(
        result,
        vec![
            "hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>",
            "IfPushFront<hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::AabbCastWrapperExternal<hknpCompressedMeshShapeInternals::ShapeCastQueryUnscaled> >",
            "<unnamed_tag>"
        ]
    );
}

#[test]
fn test_type_918() {
    let result = parse_type(
        "typedef hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::UseTwoWayBinary<hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::AllPairsWrapper<hkgpBooleanImpl::NodePair::Collector> >::<unnamed_tag> hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::IfSelect<hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::RayCastWrapper<WrappedRaycastQuery<hkcdStaticTree::DefaultTreeStorage6> > >::<unnamed_tag>;",
    );
    assert_eq!(
        result,
        vec![
            "hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>",
            "IfSelect<hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::RayCastWrapper<WrappedRaycastQuery<hkcdStaticTree::DefaultTreeStorage6> > >",
            "<unnamed_tag>"
        ]
    );
}

#[test]
fn test_type_919() {
    let result = parse_type(
        "typedef hkdCutOutFracture_DefaultStruct _A0x64909536::hkdCutOutFracture_DefaultStruct;",
    );
    assert_eq!(
        result,
        vec!["_A0x64909536", "hkdCutOutFracture_DefaultStruct"]
    );
}

#[test]
fn test_type_920() {
    let result = parse_type(
        "typedef hkgpTriangulatorType<hkContainerHeapAllocator,hkgpTriangulatorBase::VertexBase,hkgpTriangulatorBase::TriangleBase,hkgpTriangulatorBase::DefaultEdgeData<hkContainerHeapAllocator>,hkgpTriangulatorBase::SparseEdgeDataPolicy<hkgpTriangulatorBase::DefaultEdgeData<hkContainerHeapAllocator>,hkContainerHeapAllocator>,-1,4,29,0>::Vertex::<unnamed_tag> hkgpTriangulatorType<hkContainerHeapAllocator,hkgpTriangulatorBase::VertexBase,hkgpTriangulatorBase::TriangleBase,hkgpTriangulatorBase::DefaultEdgeData<hkContainerHeapAllocator>,hkgpTriangulatorBase::SparseEdgeDataPolicy<hkgpTriangulatorBase::DefaultEdgeData<hkContainerHeapAllocator>,hkContainerHeapAllocator>,-1,4,15,0>::Vertex::<unnamed_tag>;",
    );
    assert_eq!(
        result,
        vec![
            "hkgpTriangulatorType<hkContainerHeapAllocator,hkgpTriangulatorBase::VertexBase,hkgpTriangulatorBase::TriangleBase,hkgpTriangulatorBase::DefaultEdgeData<hkContainerHeapAllocator>,hkgpTriangulatorBase::SparseEdgeDataPolicy<hkgpTriangulatorBase::DefaultEdgeData<hkContainerHeapAllocator>,hkContainerHeapAllocator>,-1,4,15,0>",
            "Vertex",
            "<unnamed_tag>"
        ]
    );
}

#[test]
fn test_type_921() {
    let result = parse_type(
        "typedef hkHandle<unsigned int,0,hkcdConvexCellsCollection::CellIdDiscriminant> CellId;",
    );
    assert_eq!(result, vec!["CellId"]);
}

#[test]
fn test_type_922() {
    let result = parse_type(
        "typedef hkHandle<unsigned short,65535,hkndConstraintUniqueId>::<unnamed_tag> hkHandle<unsigned short,65535,hkSkinnedMeshShape::BoneSetIdDiscriminant>::<unnamed_tag>;",
    );
    assert_eq!(
        result,
        vec![
            "hkHandle<unsigned short,65535,hkSkinnedMeshShape::BoneSetIdDiscriminant>",
            "<unnamed_tag>"
        ]
    );
}

#[test]
fn test_type_923() {
    let result = parse_type("typedef hkMxMaskf<4>::<unnamed_tag> hkMxVectorf<8>::<unnamed_tag>;");
    assert_eq!(result, vec!["hkMxVectorf<8>", "<unnamed_tag>"]);
}

#[test]
fn test_type_924() {
    let result = parse_type(
        "typedef hkndSliceFracture_DefaultStruct _A0xb4945271::hkndSliceFracture_DefaultStruct;",
    );
    assert_eq!(
        result,
        vec!["_A0xb4945271", "hkndSliceFracture_DefaultStruct"]
    );
}

#[test]
fn test_type_925() {
    let result = parse_type("typedef hknpActivationMode::Enum hknpActivationBehavior::Enum;");
    assert_eq!(result, vec!["hknpActivationBehavior", "Enum"]);
}

#[test]
fn test_type_926() {
    let result = parse_type("typedef hknpCollisionDispatchType::Enum hkndBreakableBody::Flags;");
    assert_eq!(result, vec!["hkndBreakableBody", "Flags"]);
}

#[test]
fn test_type_927() {
    let result = parse_type("typedef hknpWorldCinfo::SolverType hkpWorldCinfo::SolverType;");
    assert_eq!(result, vec!["hkpWorldCinfo", "SolverType"]);
}

#[test]
fn test_type_928() {
    let result = parse_type(
        "typedef hkpJacobianSchemaInfo::SingleContact::<unnamed_tag> hkpJacobianSchemaInfo::WheelFriction::<unnamed_tag>;",
    );
    assert_eq!(
        result,
        vec!["hkpJacobianSchemaInfo", "WheelFriction", "<unnamed_tag>"]
    );
}

#[test]
fn test_type_929() {
    let result = parse_type(
        "typedef hkSignal2<hknpEventHandlerInput const &,hknpEvent const &>::MemberSlot<CDamageListener,void (__cdecl CDamageListener::*)(hknpEventHandlerInput const &,hknpEvent const &)>::<unnamed_tag> hkSignal0::MemberSlot<hknpShapeManager::MutableShapeInfo,void (__cdecl hknpShapeManager::MutableShapeInfo::*)(void)>::<unnamed_tag>;",
    );
    assert_eq!(
        result,
        vec![
            "hkSignal0",
            "MemberSlot<hknpShapeManager::MutableShapeInfo,void (__cdecl hknpShapeManager::MutableShapeInfo::*)(void)>",
            "<unnamed_tag>"
        ]
    );
}

#[test]
fn test_type_930() {
    let result = parse_type(
        "typedef hkSimdInt<256>::<unnamed_tag> hkcdPlanarGeometryPrimitives::NumBitsMul<hkndExactFractureEngine::NumBitsQuaternion,hkndExactFractureEngine::NumBitsQuaternion>::<unnamed_tag>;",
    );
    assert_eq!(
        result,
        vec![
            "hkcdPlanarGeometryPrimitives",
            "NumBitsMul<hkndExactFractureEngine::NumBitsQuaternion,hkndExactFractureEngine::NumBitsQuaternion>",
            "<unnamed_tag>"
        ]
    );
}

#[test]
fn test_type_931() {
    let result = parse_type(
        "typedef hkSizeOfTypeOrVoid<int>::<unnamed_tag> hkSizeOfTypeOrVoid<EdgeForEdgeDuplicateCheck>::<unnamed_tag>;",
    );
    assert_eq!(
        result,
        vec![
            "hkSizeOfTypeOrVoid<EdgeForEdgeDuplicateCheck>",
            "<unnamed_tag>"
        ]
    );
}

#[test]
fn test_type_932() {
    let result = parse_type(
        "typedef hkSizeOfTypeOrVoid<int>::<unnamed_tag> hkSizeOfTypeOrVoid<hkgpIndexedMesh::EdgeMatch>::<unnamed_tag>;",
    );
    assert_eq!(
        result,
        vec![
            "hkSizeOfTypeOrVoid<hkgpIndexedMesh::EdgeMatch>",
            "<unnamed_tag>"
        ]
    );
}

#[test]
fn test_type_933() {
    let result = parse_type(
        "typedef hkSizeOfTypeOrVoid<int>::<unnamed_tag> hkSizeOfTypeOrVoid<hkRefPtr<hkxSkinBinding> >::<unnamed_tag>;",
    );
    assert_eq!(
        result,
        vec![
            "hkSizeOfTypeOrVoid<hkRefPtr<hkxSkinBinding> >",
            "<unnamed_tag>"
        ]
    );
}

#[test]
fn test_type_934() {
    let result =
        parse_type("typedef hkStreamWriter::SeekWhence hkSeekableStreamReader::SeekWhence;");
    assert_eq!(result, vec!["hkSeekableStreamReader", "SeekWhence"]);
}

#[test]
fn test_type_935() {
    let result = parse_type("typedef hkTrait::TraitBool<0> TypeIsClass;");
    assert_eq!(result, vec!["TypeIsClass"]);
}

#[test]
fn test_type_936() {
    let result = parse_type("typedef IDirectInputEffect *LPDIRECTINPUTEFFECT;");
    assert_eq!(result, vec!["LPDIRECTINPUTEFFECT"]);
}

#[test]
fn test_type_937() {
    let result = parse_type("typedef int InternalInt;");
    assert_eq!(result, vec!["InternalInt"]);
}

#[test]
fn test_type_938() {
    let result =
        parse_type("typedef NAr::detail::<lambda116> NAr::detail::_A0xa079f785::<lambda116>;");
    assert_eq!(result, vec!["NAr", "detail", "_A0xa079f785", "<lambda116>"]);
}

#[test]
fn test_type_939() {
    let result = parse_type(
        "typedef NFMODStudioDSP_Ratchet::EParameter NFMODStudioDSP_PolyphonyLimiter::EParameter;",
    );
    assert_eq!(
        result,
        vec!["NFMODStudioDSP_PolyphonyLimiter", "EParameter"]
    );
}

#[test]
fn test_type_940() {
    let result = parse_type(
        "typedef NGSONodes::SpawnRule::__l2::EFields NGSONodes::WaitForShapeTrigger::__l2::EFields;",
    );
    assert_eq!(
        result,
        vec!["NGSONodes", "WaitForShapeTrigger", "__l2", "EFields"]
    );
}

#[test]
fn test_type_941() {
    let result = parse_type("typedef NL_DAD_STATE IP_DAD_STATE;");
    assert_eq!(result, vec!["IP_DAD_STATE"]);
}

#[test]
fn test_type_942() {
    let result = parse_type("typedef NSaveLoad::<lambda14> NSaveLoad::_A0xe56937a7::<lambda14>;");
    assert_eq!(result, vec!["NSaveLoad", "_A0xe56937a7", "<lambda14>"]);
}

#[test]
fn test_type_943() {
    let result = parse_type("typedef NVehicle::EDoorSlot CSniperAimer::EState;");
    assert_eq!(result, vec!["CSniperAimer", "EState"]);
}

#[test]
fn test_type_944() {
    let result = parse_type("typedef PPM_WMI_IDLE_STATE *PPPM_WMI_IDLE_STATE;");
    assert_eq!(result, vec!["PPPM_WMI_IDLE_STATE"]);
}

#[test]
fn test_type_945() {
    let result = parse_type(
        "typedef Scaleform::GFx::AS3::Classes::fl::Object::<unnamed_tag> Scaleform::GFx::AS3::Classes::fl::int_::<unnamed_tag>;",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "GFx",
            "AS3",
            "Classes",
            "fl",
            "int_",
            "<unnamed_tag>"
        ]
    );
}

#[test]
fn test_type_946() {
    let result = parse_type(
        "typedef Scaleform::GFx::AS3::Classes::fl::Object::<unnamed_tag> Scaleform::GFx::AS3::InstanceTraits::fl_events::TouchEvent::<unnamed_tag>;",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "GFx",
            "AS3",
            "InstanceTraits",
            "fl_events",
            "TouchEvent",
            "<unnamed_tag>"
        ]
    );
}

#[test]
fn test_type_947() {
    let result = parse_type(
        "typedef Scaleform::GFx::AS3::Classes::fl::Object::<unnamed_tag> Scaleform::GFx::AS3::InstanceTraits::fl_system::ApplicationDomain::<unnamed_tag>;",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "GFx",
            "AS3",
            "InstanceTraits",
            "fl_system",
            "ApplicationDomain",
            "<unnamed_tag>"
        ]
    );
}

#[test]
fn test_type_948() {
    let result = parse_type(
        "typedef Scaleform::GFx::AS3::Instances::fl_events::Event::MethodID Scaleform::GFx::AS3::Instances::fl_filters::BlurFilter::MethodID;",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "GFx",
            "AS3",
            "Instances",
            "fl_filters",
            "BlurFilter",
            "MethodID"
        ]
    );
}

#[test]
fn test_type_949() {
    let result = parse_type(
        "typedef Scaleform::GFx::AS3::InstanceTraits::fl_desktop::NativeDragOptions::<unnamed_tag> Scaleform::GFx::AS3::InstanceTraits::fl_events::NativeDragEvent::<unnamed_tag>;",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "GFx",
            "AS3",
            "InstanceTraits",
            "fl_events",
            "NativeDragEvent",
            "<unnamed_tag>"
        ]
    );
}

#[test]
fn test_type_950() {
    let result = parse_type(
        "typedef Scaleform::GFx::ImageFileHandlerRegistry::InitType Scaleform::Render::Rect<unsigned long>::NoInitType;",
    );
    assert_eq!(
        result,
        vec!["Scaleform", "Render", "Rect<unsigned long>", "NoInitType"]
    );
}

#[test]
fn test_type_951() {
    let result = parse_type(
        "typedef Scaleform::HashSetBase<Scaleform::GFx::FontManager::NodePtr,Scaleform::GFx::FontManager::NodePtrHashOp,Scaleform::GFx::FontManager::NodePtrHashOp,Scaleform::AllocatorLH<Scaleform::GFx::FontManager::NodePtr,2>,Scaleform::HashsetCachedEntry<Scaleform::GFx::FontManager::NodePtr,Scaleform::GFx::FontManager::NodePtrHashOp> >::<unnamed_tag> Scaleform::HashSetBase<Scaleform::GFx::AS3::SPtr<Scaleform::GFx::AS3::Instances::fl::GlobalObjectScript>,Scaleform::FixedSizeHash<Scaleform::GFx::AS3::SPtr<Scaleform::GFx::AS3::Instances::fl::GlobalObjectScript> >,Scaleform::FixedSizeHash<Scaleform::GFx::AS3::SPtr<Scaleform::GFx::AS3::Instances::fl::GlobalObjectScript> >,Scaleform::AllocatorLH<Scaleform::GFx::AS3::SPtr<Scaleform::GFx::AS3::Instances::fl::GlobalObjectScript>,341>,Scaleform::HashsetCachedEntry<Scaleform::GFx::AS3::SPtr<Scaleform::GFx::AS3::Instances::fl::GlobalObjectScript>,Scaleform::FixedSizeHash<Scaleform::GFx::AS3::SPtr<Scaleform::GFx::AS3::Instances::fl::GlobalObjectScript> > > >::<unnamed_tag>;",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "HashSetBase<Scaleform::GFx::AS3::SPtr<Scaleform::GFx::AS3::Instances::fl::GlobalObjectScript>,Scaleform::FixedSizeHash<Scaleform::GFx::AS3::SPtr<Scaleform::GFx::AS3::Instances::fl::GlobalObjectScript> >,Scaleform::FixedSizeHash<Scaleform::GFx::AS3::SPtr<Scaleform::GFx::AS3::Instances::fl::GlobalObjectScript> >,Scaleform::AllocatorLH<Scaleform::GFx::AS3::SPtr<Scaleform::GFx::AS3::Instances::fl::GlobalObjectScript>,341>,Scaleform::HashsetCachedEntry<Scaleform::GFx::AS3::SPtr<Scaleform::GFx::AS3::Instances::fl::GlobalObjectScript>,Scaleform::FixedSizeHash<Scaleform::GFx::AS3::SPtr<Scaleform::GFx::AS3::Instances::fl::GlobalObjectScript> > > >",
            "<unnamed_tag>"
        ]
    );
}

#[test]
fn test_type_952() {
    let result = parse_type(
        "typedef Scaleform::HashSetBase<Scaleform::GFx::FontManager::NodePtr,Scaleform::GFx::FontManager::NodePtrHashOp,Scaleform::GFx::FontManager::NodePtrHashOp,Scaleform::AllocatorLH<Scaleform::GFx::FontManager::NodePtr,2>,Scaleform::HashsetCachedEntry<Scaleform::GFx::FontManager::NodePtr,Scaleform::GFx::FontManager::NodePtrHashOp> >::<unnamed_tag> Scaleform::HashSetBase<Scaleform::HashNode<Scaleform::GFx::AS3::Instances::fl::ConstStringKey,Scaleform::GFx::AS3::ClassInfo const *,Scaleform::GFx::AS3::Instances::fl::ConstStringHashFn>,Scaleform::HashNode<Scaleform::GFx::AS3::Instances::fl::ConstStringKey,Scaleform::GFx::AS3::ClassInfo const *,Scaleform::GFx::AS3::Instances::fl::ConstStringHashFn>::NodeHashF,Scaleform::HashNode<Scaleform::GFx::AS3::Instances::fl::ConstStringKey,Scaleform::GFx::AS3::ClassInfo const *,Scaleform::GFx::AS3::Instances::fl::ConstStringHashFn>::NodeAltHashF,Scaleform::AllocatorGH<Scaleform::GFx::AS3::Instances::fl::ConstStringKey,328>,Scaleform::HashsetCachedNodeEntry<Scaleform::HashNode<Scaleform::GFx::AS3::Instances::fl::ConstStringKey,Scaleform::GFx::AS3::ClassInfo const *,Scaleform::GFx::AS3::Instances::fl::ConstStringHashFn>,Scaleform::HashNode<Scaleform::GFx::AS3::Instances::fl::ConstStringKey,Scaleform::GFx::AS3::ClassInfo const *,Scaleform::GFx::AS3::Instances::fl::ConstStringHashFn>::NodeHashF> >::<unnamed_tag>;",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "HashSetBase<Scaleform::HashNode<Scaleform::GFx::AS3::Instances::fl::ConstStringKey,Scaleform::GFx::AS3::ClassInfo const *,Scaleform::GFx::AS3::Instances::fl::ConstStringHashFn>,Scaleform::HashNode<Scaleform::GFx::AS3::Instances::fl::ConstStringKey,Scaleform::GFx::AS3::ClassInfo const *,Scaleform::GFx::AS3::Instances::fl::ConstStringHashFn>::NodeHashF,Scaleform::HashNode<Scaleform::GFx::AS3::Instances::fl::ConstStringKey,Scaleform::GFx::AS3::ClassInfo const *,Scaleform::GFx::AS3::Instances::fl::ConstStringHashFn>::NodeAltHashF,Scaleform::AllocatorGH<Scaleform::GFx::AS3::Instances::fl::ConstStringKey,328>,Scaleform::HashsetCachedNodeEntry<Scaleform::HashNode<Scaleform::GFx::AS3::Instances::fl::ConstStringKey,Scaleform::GFx::AS3::ClassInfo const *,Scaleform::GFx::AS3::Instances::fl::ConstStringHashFn>,Scaleform::HashNode<Scaleform::GFx::AS3::Instances::fl::ConstStringKey,Scaleform::GFx::AS3::ClassInfo const *,Scaleform::GFx::AS3::Instances::fl::ConstStringHashFn>::NodeHashF> >",
            "<unnamed_tag>"
        ]
    );
}

#[test]
fn test_type_953() {
    let result = parse_type(
        "typedef Scaleform::HashSetBase<Scaleform::GFx::FontManager::NodePtr,Scaleform::GFx::FontManager::NodePtrHashOp,Scaleform::GFx::FontManager::NodePtrHashOp,Scaleform::AllocatorLH<Scaleform::GFx::FontManager::NodePtr,2>,Scaleform::HashsetCachedEntry<Scaleform::GFx::FontManager::NodePtr,Scaleform::GFx::FontManager::NodePtrHashOp> >::<unnamed_tag> Scaleform::HashSetBase<Scaleform::HashNode<unsigned long,Scaleform::GFx::AS3::TypeBarrier *,Scaleform::FixedSizeHash<unsigned long> >,Scaleform::HashNode<unsigned long,Scaleform::GFx::AS3::TypeBarrier *,Scaleform::FixedSizeHash<unsigned long> >::NodeHashF,Scaleform::HashNode<unsigned long,Scaleform::GFx::AS3::TypeBarrier *,Scaleform::FixedSizeHash<unsigned long> >::NodeAltHashF,Scaleform::AllocatorLH<unsigned long,341>,Scaleform::HashsetCachedNodeEntry<Scaleform::HashNode<unsigned long,Scaleform::GFx::AS3::TypeBarrier *,Scaleform::FixedSizeHash<unsigned long> >,Scaleform::HashNode<unsigned long,Scaleform::GFx::AS3::TypeBarrier *,Scaleform::FixedSizeHash<unsigned long> >::NodeHashF> >::<unnamed_tag>;",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "HashSetBase<Scaleform::HashNode<unsigned long,Scaleform::GFx::AS3::TypeBarrier *,Scaleform::FixedSizeHash<unsigned long> >,Scaleform::HashNode<unsigned long,Scaleform::GFx::AS3::TypeBarrier *,Scaleform::FixedSizeHash<unsigned long> >::NodeHashF,Scaleform::HashNode<unsigned long,Scaleform::GFx::AS3::TypeBarrier *,Scaleform::FixedSizeHash<unsigned long> >::NodeAltHashF,Scaleform::AllocatorLH<unsigned long,341>,Scaleform::HashsetCachedNodeEntry<Scaleform::HashNode<unsigned long,Scaleform::GFx::AS3::TypeBarrier *,Scaleform::FixedSizeHash<unsigned long> >,Scaleform::HashNode<unsigned long,Scaleform::GFx::AS3::TypeBarrier *,Scaleform::FixedSizeHash<unsigned long> >::NodeHashF> >",
            "<unnamed_tag>"
        ]
    );
}

#[test]
fn test_type_954() {
    let result = parse_type(
        "typedef Scaleform::RefCountBase<Scaleform::GFx::AMP::FunctionDesc,578>::<unnamed_tag> Scaleform::RefCountBase<Scaleform::Render::Matrix4x4<float>,2>::<unnamed_tag>;",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "RefCountBase<Scaleform::Render::Matrix4x4<float>,2>",
            "<unnamed_tag>"
        ]
    );
}

#[test]
fn test_type_955() {
    let result = parse_type(
        "typedef Scaleform::RefCountBase<Scaleform::GFx::AMP::FunctionDesc,578>::<unnamed_tag> Scaleform::RefCountBase<Scaleform::Render::Text::DocView,74>::<unnamed_tag>;",
    );
    assert_eq!(
        result,
        vec![
            "Scaleform",
            "RefCountBase<Scaleform::Render::Text::DocView,74>",
            "<unnamed_tag>"
        ]
    );
}

#[test]
fn test_type_956() {
    let result = parse_type("typedef SHttpServer *HHttpServer;");
    assert_eq!(result, vec!["HHttpServer"]);
}

#[test]
fn test_type_957() {
    let result = parse_type(
        "typedef std::_Tmap_traits<std::string,CUpdateQueue::MeanTracker *,std::less<std::string >,std::allocator<std::pair<std::string const ,CUpdateQueue::MeanTracker *> >,0>::<unnamed_tag> std::_Tmap_traits<unsigned int,bool,std::less<unsigned int>,std::allocator<std::pair<unsigned int const ,bool> >,0>::<unnamed_tag>;",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "_Tmap_traits<unsigned int,bool,std::less<unsigned int>,std::allocator<std::pair<unsigned int const ,bool> >,0>",
            "<unnamed_tag>"
        ]
    );
}

#[test]
fn test_type_958() {
    let result = parse_type(
        "typedef std::_Tree_val<std::_Tmap_traits<std::string,CUpdateQueue::MeanTracker *,std::less<std::string >,std::allocator<std::pair<std::string const ,CUpdateQueue::MeanTracker *> >,0> >::_Redbl std::_Tree_val<std::_Tmap_traits<ava::idstring<NAnimationSystem::SAnimationIdTag,0>,int,std::less<ava::idstring<NAnimationSystem::SAnimationIdTag,0> >,std::allocator<std::pair<ava::idstring<NAnimationSystem::SAnimationIdTag,0> const ,int> >,0> >::_Redbl;",
    );
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
fn test_type_959() {
    let result = parse_type(
        "typedef std::_Tree_val<std::_Tmap_traits<std::string,CUpdateQueue::MeanTracker *,std::less<std::string >,std::allocator<std::pair<std::string const ,CUpdateQueue::MeanTracker *> >,0> >::_Redbl std::_Tree_val<std::_Tmap_traits<unsigned int,std::string,std::less<unsigned int>,Graphics::CustomAllocator<std::pair<unsigned int const ,std::string >,15>,1> >::_Redbl;",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "_Tree_val<std::_Tmap_traits<unsigned int,std::string,std::less<unsigned int>,Graphics::CustomAllocator<std::pair<unsigned int const ,std::string >,15>,1> >",
            "_Redbl"
        ]
    );
}

#[test]
fn test_type_960() {
    let result = parse_type(
        "typedef std::_Tree_val<std::_Tmap_traits<std::string,CUpdateQueue::MeanTracker *,std::less<std::string >,std::allocator<std::pair<std::string const ,CUpdateQueue::MeanTracker *> >,0> >::_Redbl std::_Tree_val<std::_Tmap_traits<unsigned short,SSceneOccluder,std::less<unsigned short>,std::allocator<std::pair<unsigned short const ,SSceneOccluder> >,0> >::_Redbl;",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "_Tree_val<std::_Tmap_traits<unsigned short,SSceneOccluder,std::less<unsigned short>,std::allocator<std::pair<unsigned short const ,SSceneOccluder> >,0> >",
            "_Redbl"
        ]
    );
}

#[test]
fn test_type_961() {
    let result = parse_type(
        "typedef std::tr1::_Callable_base<<lambda5>,0>::<unnamed_tag> std::tr1::_Callable_base<<lambda16>,0>::<unnamed_tag>;",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "tr1",
            "_Callable_base<<lambda16>,0>",
            "<unnamed_tag>"
        ]
    );
}

#[test]
fn test_type_962() {
    let result = parse_type(
        "typedef std::tr1::_Callable_base<<lambda5>,0>::<unnamed_tag> std::tr1::_Callable_base<<lambda176>,0>::<unnamed_tag>;",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "tr1",
            "_Callable_base<<lambda176>,0>",
            "<unnamed_tag>"
        ]
    );
}

#[test]
fn test_type_963() {
    let result = parse_type(
        "typedef std::tr1::_Callable_base<<lambda5>,0>::<unnamed_tag> std::tr1::_Callable_base<<lambda40>,0>::<unnamed_tag>;",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "tr1",
            "_Callable_base<<lambda40>,0>",
            "<unnamed_tag>"
        ]
    );
}

#[test]
fn test_type_964() {
    let result = parse_type(
        "typedef std::tr1::_Callable_base<<lambda5>,0>::<unnamed_tag> std::tr1::_Callable_base<<lambda63>,0>::<unnamed_tag>;",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "tr1",
            "_Callable_base<<lambda63>,0>",
            "<unnamed_tag>"
        ]
    );
}

#[test]
fn test_type_965() {
    let result = parse_type(
        "typedef std::tr1::_Callable_base<<lambda5>,0>::<unnamed_tag> std::tr1::_Callable_base<NGraphicsEngine::<lambda1>,0>::<unnamed_tag>;",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "tr1",
            "_Callable_base<NGraphicsEngine::<lambda1>,0>",
            "<unnamed_tag>"
        ]
    );
}

#[test]
fn test_type_966() {
    let result = parse_type(
        "typedef std::tr1::_Callable_base<<lambda5>,0>::<unnamed_tag> std::tr1::_Callable_base<void (__cdecl NSaveLoad::CManagedObject<NSaveLoad::CLocalDataObject<unsigned int,NSaveLoad::DefaultObjectFactory> >::*const)(void),0>::<unnamed_tag>;",
    );
    assert_eq!(
        result,
        vec![
            "std",
            "tr1",
            "_Callable_base<void (__cdecl NSaveLoad::CManagedObject<NSaveLoad::CLocalDataObject<unsigned int,NSaveLoad::DefaultObjectFactory> >::*const)(void),0>",
            "<unnamed_tag>"
        ]
    );
}

#[test]
fn test_type_967() {
    let result = parse_type(
        "typedef std::tr1::array<std::deque<COperation *>,5>::<unnamed_tag> std::tr1::array<float,16>::<unnamed_tag>;",
    );
    assert_eq!(
        result,
        vec!["std", "tr1", "array<float,16>", "<unnamed_tag>"]
    );
}

#[test]
fn test_type_968() {
    let result = parse_type(
        "typedef Steam::MusicPlayerWantsPlayPrevious_t::<unnamed_tag> MusicPlayerSelectsQueueEntry_t::<unnamed_tag>;",
    );
    assert_eq!(
        result,
        vec!["MusicPlayerSelectsQueueEntry_t", "<unnamed_tag>"]
    );
}

#[test]
fn test_type_969() {
    let result = parse_type(
        "typedef Steam::MusicPlayerWantsPlayPrevious_t::<unnamed_tag> Steam::HTML_VerticalScroll_t::<unnamed_tag>;",
    );
    assert_eq!(
        result,
        vec!["Steam", "HTML_VerticalScroll_t", "<unnamed_tag>"]
    );
}

#[test]
fn test_type_970() {
    let result = parse_type("typedef tagBIND_OPTS BIND_OPTS;");
    assert_eq!(result, vec!["BIND_OPTS"]);
}

#[test]
fn test_type_971() {
    let result = parse_type("typedef tagCOMPOSITIONFORM *LPCOMPOSITIONFORM;");
    assert_eq!(result, vec!["LPCOMPOSITIONFORM"]);
}

#[test]
fn test_type_972() {
    let result = parse_type("typedef tagDRAWITEMSTRUCT *PDRAWITEMSTRUCT;");
    assert_eq!(result, vec!["PDRAWITEMSTRUCT"]);
}

#[test]
fn test_type_973() {
    let result = parse_type("typedef tagHW_PROFILE_INFOA *LPHW_PROFILE_INFOA;");
    assert_eq!(result, vec!["LPHW_PROFILE_INFOA"]);
}

#[test]
fn test_type_974() {
    let result = parse_type("typedef tagSTYLEBUFW STYLEBUFW;");
    assert_eq!(result, vec!["STYLEBUFW"]);
}

#[test]
fn test_type_975() {
    let result = parse_type("typedef tagWNDCLASSEXW WNDCLASSEXW;");
    assert_eq!(result, vec!["WNDCLASSEXW"]);
}

#[test]
fn test_type_976() {
    let result = parse_type("typedef union $D91DE99308EDB4B15B954CC4B13FA8FA _Dconst;");
    assert_eq!(result, vec!["_Dconst"]);
}

#[test]
fn test_type_977() {
    let result = parse_type("typedef unsigned __int16 **png_uint_16pp;");
    assert_eq!(result, vec!["png_uint_16pp"]);
}

#[test]
fn test_type_978() {
    let result = parse_type("typedef unsigned __int16 TriangleIndex;");
    assert_eq!(result, vec!["TriangleIndex"]);
}

#[test]
fn test_type_979() {
    let result = parse_type("typedef unsigned __int16 TTerrainPatchDispType;");
    assert_eq!(result, vec!["TTerrainPatchDispType"]);
}

#[test]
fn test_type_980() {
    let result = parse_type("typedef unsigned __int8 *PUCHAR;");
    assert_eq!(result, vec!["PUCHAR"]);
}

#[test]
fn test_type_981() {
    let result = parse_type("typedef unsigned __int8 XML_Bool;");
    assert_eq!(result, vec!["XML_Bool"]);
}

#[test]
fn test_type_982() {
    let result = parse_type("typedef unsigned int tval_t;");
    assert_eq!(result, vec!["tval_t"]);
}

#[test]
fn test_type_983() {
    let result = parse_type("typedef unsigned int uint32;");
    assert_eq!(result, vec!["uint32"]);
}

#[test]
fn test_type_984() {
    let result = parse_type("typedef unsigned int WICColor;");
    assert_eq!(result, vec!["WICColor"]);
}

#[test]
fn test_type_985() {
    let result = parse_type("typedef void (__fastcall *GFSDK_WAVEWORKS_FREE)(void *);");
    assert_eq!(result, vec!["GFSDK_WAVEWORKS_FREE"]);
}

#[test]
fn test_type_986() {
    let result = parse_type("typedef void **PHANDLE;");
    assert_eq!(result, vec!["PHANDLE"]);
}

#[test]
fn test_type_987() {
    let result = parse_type("typedef XML_ParserStruct *XML_Parser;");
    assert_eq!(result, vec!["XML_Parser"]);
}

#[test]
fn test_type_988() {
    let result = parse_type(
        "union __declspec(align(16)) getVtablehkndSplitByPhysicsIslandsAction::__l2::<unnamed_type_u>",
    );
    assert_eq!(
        result,
        vec![
            "getVtablehkndSplitByPhysicsIslandsAction",
            "__l2",
            "<unnamed_type_u>"
        ]
    );
}

#[test]
fn test_type_989() {
    let result = parse_type("union __m128");
    assert_eq!(result, vec!["__m128"]);
}

#[test]
fn test_type_990() {
    let result = parse_type("union _TP_CALLBACK_ENVIRON_V3::<unnamed_type_u>");
    assert_eq!(result, vec!["_TP_CALLBACK_ENVIRON_V3", "<unnamed_type_u>"]);
}

#[test]
fn test_type_991() {
    let result = parse_type("union $51741BED59428C0B4AE983CCA8A73120");
    assert_eq!(result, vec!["$51741BED59428C0B4AE983CCA8A73120"]);
}

#[test]
fn test_type_992() {
    let result = parse_type("union $D14D394676A1F3C855ED2D6CEB6BACFD");
    assert_eq!(result, vec!["$D14D394676A1F3C855ED2D6CEB6BACFD"]);
}

#[test]
fn test_type_993() {
    let result = parse_type("union getVtablehkaiConvexSilhouetteSet::__l2::<unnamed_type_u>");
    assert_eq!(
        result,
        vec![
            "getVtablehkaiConvexSilhouetteSet",
            "__l2",
            "<unnamed_type_u>"
        ]
    );
}

#[test]
fn test_type_994() {
    let result = parse_type("union getVtablehkdBreakableShapeContactArea::__l2::<unnamed_type_u>");
    assert_eq!(
        result,
        vec![
            "getVtablehkdBreakableShapeContactArea",
            "__l2",
            "<unnamed_type_u>"
        ]
    );
}

#[test]
fn test_type_995() {
    let result = parse_type("union getVtablehkndShockWaveAction::__l2::<unnamed_type_u>");
    assert_eq!(
        result,
        vec!["getVtablehkndShockWaveAction", "__l2", "<unnamed_type_u>"]
    );
}

#[test]
fn test_type_996() {
    let result = parse_type("union simd_extract_fp::__l2::<unnamed_type_u>");
    assert_eq!(result, vec!["simd_extract_fp", "__l2", "<unnamed_type_u>"]);
}
