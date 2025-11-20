use crate::type_parser::parse_type;

#[test]
fn test_type_001() {
    let result = parse_type("typedef union $D91DE99308EDB4B15B954CC4B13FA8FA _Dconst;");
    assert_eq!(result, vec!["_Dconst"]);
}

#[test]
fn test_type_002() {
    let result = parse_type("enum SProfileOffer::OfferType : __int32");
    assert_eq!(result, vec!["SProfileOffer", "OfferType"]);
}

#[test]
fn test_type_003() {
    let result = parse_type("enum Scaleform::Render::PrimitiveFillFlags : __int32");
    assert_eq!(result, vec!["Scaleform", "Render", "PrimitiveFillFlags"]);
}

#[test]
fn test_type_004() {
    let result = parse_type("enum NVehicle::EDoorSlot : __int32");
    assert_eq!(result, vec!["NVehicle", "EDoorSlot"]);
}

#[test]
fn test_type_005() {
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
fn test_type_006() {
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
fn test_type_007() {
    let result = parse_type(
        "typedef hkArrayBase<boost::shared_ptr<CAnimationInstance> >::<unnamed_tag> hkArrayBase<hkResource *>::<unnamed_tag>;",
    );
    assert_eq!(result, vec!["hkArrayBase<hkResource *>", "<unnamed_tag>"]);
}

#[test]
fn test_type_008() {
    let result = parse_type("enum Scaleform::GFx::DisplayObjectBase::TopMostResult : __int32");
    assert_eq!(
        result,
        vec!["Scaleform", "GFx", "DisplayObjectBase", "TopMostResult"]
    );
}

#[test]
fn test_type_009() {
    let result = parse_type("enum Scaleform::GFx::TextKeyMap::KeyState : __int32");
    assert_eq!(result, vec!["Scaleform", "GFx", "TextKeyMap", "KeyState"]);
}

#[test]
fn test_type_010() {
    let result = parse_type(
        "typedef Scaleform::GFx::ImageFileHandlerRegistry::InitType Scaleform::Render::Rect<unsigned long>::NoInitType;",
    );
    assert_eq!(
        result,
        vec!["Scaleform", "Render", "Rect<unsigned long>", "NoInitType"]
    );
}

#[test]
fn test_type_011() {
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
fn test_type_012() {
    let result = parse_type("enum NAnimationSystem::CAnimationTrack::EPhaseMatchMode : __int32");
    assert_eq!(
        result,
        vec!["NAnimationSystem", "CAnimationTrack", "EPhaseMatchMode"]
    );
}

#[test]
fn test_type_013() {
    let result = parse_type("typedef hknpActivationMode::Enum hknpActivationBehavior::Enum;");
    assert_eq!(result, vec!["hknpActivationBehavior", "Enum"]);
}

#[test]
fn test_type_014() {
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
fn test_type_015() {
    let result = parse_type("enum CUIBase::State : __int32");
    assert_eq!(result, vec!["CUIBase", "State"]);
}

#[test]
fn test_type_016() {
    let result = parse_type("enum ETextureCacheResult : __int32");
    assert_eq!(result, vec!["ETextureCacheResult"]);
}

#[test]
fn test_type_017() {
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
fn test_type_018() {
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
fn test_type_019() {
    let result =
        parse_type("typedef hkStreamWriter::SeekWhence hkSeekableStreamReader::SeekWhence;");
    assert_eq!(result, vec!["hkSeekableStreamReader", "SeekWhence"]);
}

#[test]
fn test_type_020() {
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
fn test_type_021() {
    let result = parse_type("enum NGSONodes::IsUpright::__l2::<unnamed_tag> : __int32");
    assert_eq!(
        result,
        vec!["NGSONodes", "IsUpright", "__l2", "<unnamed_tag>"]
    );
}

#[test]
fn test_type_022() {
    let result = parse_type("enum NGSONodes::TriggerWingsuit::__l2::EFields : __int32");
    assert_eq!(
        result,
        vec!["NGSONodes", "TriggerWingsuit", "__l2", "EFields"]
    );
}

#[test]
fn test_type_023() {
    let result = parse_type("typedef NVehicle::EDoorSlot CSniperAimer::EState;");
    assert_eq!(result, vec!["CSniperAimer", "EState"]);
}

#[test]
fn test_type_024() {
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
fn test_type_025() {
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
fn test_type_026() {
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
fn test_type_027() {
    let result = parse_type("enum EMemoryRange : __int32");
    assert_eq!(result, vec!["EMemoryRange"]);
}

#[test]
fn test_type_028() {
    let result = parse_type(
        "typedef Steam::MusicPlayerWantsPlayPrevious_t::<unnamed_tag> MusicPlayerSelectsQueueEntry_t::<unnamed_tag>;",
    );
    assert_eq!(
        result,
        vec!["MusicPlayerSelectsQueueEntry_t", "<unnamed_tag>"]
    );
}

#[test]
fn test_type_029() {
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
fn test_type_030() {
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
fn test_type_031() {
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
fn test_type_032() {
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
fn test_type_033() {
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
fn test_type_034() {
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
fn test_type_035() {
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
fn test_type_036() {
    let result = parse_type(
        "typedef hkpJacobianSchemaInfo::SingleContact::<unnamed_tag> hkpJacobianSchemaInfo::WheelFriction::<unnamed_tag>;",
    );
    assert_eq!(
        result,
        vec!["hkpJacobianSchemaInfo", "WheelFriction", "<unnamed_tag>"]
    );
}

#[test]
fn test_type_037() {
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
fn test_type_038() {
    let result = parse_type(
        "typedef FleeGraph::<unnamed_tag> hkaiHierarchicalGraphHeuristic::<unnamed_tag>;",
    );
    assert_eq!(
        result,
        vec!["hkaiHierarchicalGraphHeuristic", "<unnamed_tag>"]
    );
}

#[test]
fn test_type_039() {
    let result = parse_type("enum hkaiDegenerateFaceCutter::<unnamed_tag> : __int32");
    assert_eq!(result, vec!["hkaiDegenerateFaceCutter", "<unnamed_tag>"]);
}

#[test]
fn test_type_040() {
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
fn test_type_041() {
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
fn test_type_042() {
    let result = parse_type("enum hkpWorld::ReintegrationRecollideMode : __int32");
    assert_eq!(result, vec!["hkpWorld", "ReintegrationRecollideMode"]);
}

#[test]
fn test_type_043() {
    let result = parse_type(
        "typedef hkaiSilhouetteGenerator::ForceGenerateOntoPpuReasons hkpCollidable::ForceCollideOntoPpuReasons;",
    );
    assert_eq!(result, vec!["hkpCollidable", "ForceCollideOntoPpuReasons"]);
}

#[test]
fn test_type_044() {
    let result = parse_type("typedef hknpWorldCinfo::SolverType hkpWorldCinfo::SolverType;");
    assert_eq!(result, vec!["hkpWorldCinfo", "SolverType"]);
}

#[test]
fn test_type_045() {
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
fn test_type_046() {
    let result = parse_type("enum Scaleform::Render::Text::HTMLElementsEnum : __int32");
    assert_eq!(
        result,
        vec!["Scaleform", "Render", "Text", "HTMLElementsEnum"]
    );
}

#[test]
fn test_type_047() {
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
fn test_type_048() {
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
fn test_type_049() {
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
fn test_type_050() {
    let result = parse_type("enum D2D1_PATH_SEGMENT : __int32");
    assert_eq!(result, vec!["D2D1_PATH_SEGMENT"]);
}

#[test]
fn test_type_051() {
    let result = parse_type("enum CForcePulse::E_MECH_PUNCH_DIR : __int32");
    assert_eq!(result, vec!["CForcePulse", "E_MECH_PUNCH_DIR"]);
}

#[test]
fn test_type_052() {
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
fn test_type_053() {
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
fn test_type_054() {
    let result = parse_type("struct __cppobj IStatisticProvider::SEntry");
    assert_eq!(result, vec!["IStatisticProvider", "SEntry"]);
}

#[test]
fn test_type_055() {
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
fn test_type_056() {
    let result = parse_type("struct /*VFT*/ OSuite::TList<OSuite::ZOEdmEntitySet *>_vtbl");
    assert_eq!(
        result,
        vec!["OSuite", "TList<OSuite::ZOEdmEntitySet *>_vtbl"]
    );
}

#[test]
fn test_type_057() {
    let result = parse_type("struct $CABC8D72D33B996952E9BFEE2726C6EC");
    assert_eq!(result, vec!["$CABC8D72D33B996952E9BFEE2726C6EC"]);
}

#[test]
fn test_type_058() {
    let result = parse_type("union __m128");
    assert_eq!(result, vec!["__m128"]);
}

#[test]
fn test_type_059() {
    let result = parse_type("struct __cppobj hkBlockStreamBase::Stream");
    assert_eq!(result, vec!["hkBlockStreamBase", "Stream"]);
}

#[test]
fn test_type_060() {
    let result = parse_type("struct __cppobj hkEnum<enum hknpWorld::SimulationStage,unsigned int>");
    assert_eq!(
        result,
        vec!["hkEnum<enum hknpWorld::SimulationStage,unsigned int>"]
    );
}

#[test]
fn test_type_061() {
    let result = parse_type(
        "struct __cppobj hkArray<hknpDeactivatedIsland *,hkContainerHeapAllocator> : hkArrayBase<hknpDeactivatedIsland *>",
    );
    assert_eq!(
        result,
        vec!["hkArray<hknpDeactivatedIsland *,hkContainerHeapAllocator>"]
    );
}

#[test]
fn test_type_062() {
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
fn test_type_063() {
    let result = parse_type("struct Graphics::HVertexProgram_t");
    assert_eq!(result, vec!["Graphics", "HVertexProgram_t"]);
}

#[test]
fn test_type_064() {
    let result = parse_type("struct __declspec(align(8)) SEffectRTParamHandler::ArrayParamHash");
    assert_eq!(result, vec!["SEffectRTParamHandler", "ArrayParamHash"]);
}

#[test]
fn test_type_065() {
    let result = parse_type(
        "struct __cppobj __declspec(align(8)) std::_Vector_val<boost::shared_ptr<CAnimationInstance>> : std::_Container_base0",
    );
    assert_eq!(
        result,
        vec!["std", "_Vector_val<boost::shared_ptr<CAnimationInstance>>"]
    );
}

#[test]
fn test_type_066() {
    let result = parse_type(
        "struct __cppobj hkEnum<enum hkpVelocityAccumulator::hkpAccumulatorType,unsigned char>",
    );
    assert_eq!(
        result,
        vec!["hkEnum<enum hkpVelocityAccumulator::hkpAccumulatorType,unsigned char>"]
    );
}

#[test]
fn test_type_067() {
    let result = parse_type("struct __cppobj hkRefPtr<hkpBreakableBody::Controller>");
    assert_eq!(result, vec!["hkRefPtr<hkpBreakableBody::Controller>"]);
}

#[test]
fn test_type_068() {
    let result = parse_type("const struct __cppobj hkdGeometry : hkReferencedObject");
    assert_eq!(result, vec!["hkdGeometry"]);
}

#[test]
fn test_type_069() {
    let result = parse_type(
        "struct __cppobj hkArray<hkdMeshGraphicsShape::SkinData,hkContainerHeapAllocator> : hkArrayBase<hkdMeshGraphicsShape::SkinData>",
    );
    assert_eq!(
        result,
        vec!["hkArray<hkdMeshGraphicsShape::SkinData,hkContainerHeapAllocator>"]
    );
}

#[test]
fn test_type_070() {
    let result = parse_type("struct __cppobj TaskQueue::detail::IExecutor");
    assert_eq!(result, vec!["TaskQueue", "detail", "IExecutor"]);
}

#[test]
fn test_type_071() {
    let result = parse_type("struct /*VFT*/ std::wstreambuf_vtbl");
    assert_eq!(result, vec!["std", "wstreambuf_vtbl"]);
}

#[test]
fn test_type_072() {
    let result = parse_type("struct /*VFT*/ std::wostream_vtbl");
    assert_eq!(result, vec!["std", "wostream_vtbl"]);
}

#[test]
fn test_type_073() {
    let result = parse_type("struct __cppobj std::tr1::bad_weak_ptr : std::exception");
    assert_eq!(result, vec!["std", "tr1", "bad_weak_ptr"]);
}

#[test]
fn test_type_074() {
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
fn test_type_075() {
    let result = parse_type("struct $B0414F1DEC363B3EFC1FC6E4B7F77361");
    assert_eq!(result, vec!["$B0414F1DEC363B3EFC1FC6E4B7F77361"]);
}

#[test]
fn test_type_076() {
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
fn test_type_077() {
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
fn test_type_078() {
    let result = parse_type("union _TP_CALLBACK_ENVIRON_V3::<unnamed_type_u>");
    assert_eq!(result, vec!["_TP_CALLBACK_ENVIRON_V3", "<unnamed_type_u>"]);
}

#[test]
fn test_type_079() {
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
fn test_type_080() {
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
fn test_type_081() {
    let result =
        parse_type("struct __cppobj Steam::HTML_OpenLinkInNewTab_t : Steam::SteamCallback_t");
    assert_eq!(result, vec!["Steam", "HTML_OpenLinkInNewTab_t"]);
}

#[test]
fn test_type_082() {
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
fn test_type_083() {
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
fn test_type_084() {
    let result = parse_type("struct /*VFT*/ Scaleform::Render::ContextImpl::RenderNotify_vtbl");
    assert_eq!(
        result,
        vec!["Scaleform", "Render", "ContextImpl", "RenderNotify_vtbl"]
    );
}

#[test]
fn test_type_085() {
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
fn test_type_086() {
    let result = parse_type("struct __cppobj Scaleform::Ptr<Scaleform::Render::BlendPrimitive>");
    assert_eq!(
        result,
        vec!["Scaleform", "Ptr<Scaleform::Render::BlendPrimitive>"]
    );
}

#[test]
fn test_type_087() {
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
fn test_type_088() {
    let result = parse_type("struct __cppobj Scaleform::Render::MeshKeySetHandle");
    assert_eq!(result, vec!["Scaleform", "Render", "MeshKeySetHandle"]);
}

#[test]
fn test_type_089() {
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
fn test_type_090() {
    let result = parse_type("struct /*VFT*/ Scaleform::Render::ProfileModifierTDensity_vtbl");
    assert_eq!(
        result,
        vec!["Scaleform", "Render", "ProfileModifierTDensity_vtbl"]
    );
}

#[test]
fn test_type_091() {
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
fn test_type_092() {
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
fn test_type_093() {
    let result = parse_type(
        "struct __cppobj Scaleform::Render::Text::DocView::DocumentText : Scaleform::Render::Text::StyledText",
    );
    assert_eq!(
        result,
        vec!["Scaleform", "Render", "Text", "DocView", "DocumentText"]
    );
}

#[test]
fn test_type_094() {
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
fn test_type_095() {
    let result = parse_type(
        "struct __cppobj std::allocator<std::pair<unsigned int const ,int> > : std::_Allocator_base<std::pair<unsigned int const ,int> >",
    );
    assert_eq!(
        result,
        vec!["std", "allocator<std::pair<unsigned int const ,int> >"]
    );
}

#[test]
fn test_type_096() {
    let result = parse_type("struct __cppobj TAdfStructPtr<SDeformPoints> : CAdfStructPtrBase");
    assert_eq!(result, vec!["TAdfStructPtr<SDeformPoints>"]);
}

#[test]
fn test_type_097() {
    let result = parse_type("const struct SVehicleDynamicLights");
    assert_eq!(result, vec!["SVehicleDynamicLights"]);
}

#[test]
fn test_type_098() {
    let result = parse_type("struct __declspec(align(8)) SDecals::Arraydecal_counts");
    assert_eq!(result, vec!["SDecals", "Arraydecal_counts"]);
}

#[test]
fn test_type_099() {
    let result = parse_type("struct __cppobj dynamo::vm::machine");
    assert_eq!(result, vec!["dynamo", "vm", "machine"]);
}

#[test]
fn test_type_100() {
    let result = parse_type("struct SEffectAttachment");
    assert_eq!(result, vec!["SEffectAttachment"]);
}

#[test]
fn test_type_101() {
    let result = parse_type("struct __cppobj __declspec(align(8)) CCharacter::SMeleeLoopEffect");
    assert_eq!(result, vec!["CCharacter", "SMeleeLoopEffect"]);
}

#[test]
fn test_type_102() {
    let result = parse_type("struct __cppobj TAdfStructPtr<SSubmersible> : CAdfStructPtrBase");
    assert_eq!(result, vec!["TAdfStructPtr<SSubmersible>"]);
}

#[test]
fn test_type_103() {
    let result = parse_type("struct __cppobj hkRefPtr<hknpCharacterRigidBodyListener>");
    assert_eq!(result, vec!["hkRefPtr<hknpCharacterRigidBodyListener>"]);
}

#[test]
fn test_type_104() {
    let result = parse_type("struct __cppobj CCustomCharacterRBListener::SLedgePoint");
    assert_eq!(result, vec!["CCustomCharacterRBListener", "SLedgePoint"]);
}

#[test]
fn test_type_105() {
    let result = parse_type("struct /*VFT*/ TAdfStructPtr<SDriverLean>_vtbl");
    assert_eq!(result, vec!["TAdfStructPtr<SDriverLean>_vtbl"]);
}

#[test]
fn test_type_106() {
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
fn test_type_107() {
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
fn test_type_108() {
    let result = parse_type("struct /*VFT*/ COnlineAppSystemFactory_vtbl");
    assert_eq!(result, vec!["COnlineAppSystemFactory_vtbl"]);
}

#[test]
fn test_type_109() {
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
fn test_type_110() {
    let result =
        parse_type("struct __cppobj COnlineObserver<CAsynchronousBragThresholdObserver<__int64> >");
    assert_eq!(
        result,
        vec!["COnlineObserver<CAsynchronousBragThresholdObserver<__int64> >"]
    );
}

#[test]
fn test_type_111() {
    let result = parse_type("struct __cppobj __declspec(align(8)) TArray<SEventID>");
    assert_eq!(result, vec!["TArray<SEventID>"]);
}

#[test]
fn test_type_112() {
    let result = parse_type("struct __cppobj STerrainStreamPatchDataInternal");
    assert_eq!(result, vec!["STerrainStreamPatchDataInternal"]);
}

#[test]
fn test_type_113() {
    let result = parse_type("struct FMOD_CODEC_WAVEFORMAT");
    assert_eq!(result, vec!["FMOD_CODEC_WAVEFORMAT"]);
}

#[test]
fn test_type_114() {
    let result = parse_type("struct __cppobj hkArrayBase<hkaiDirectedGraphExplicitCost::Edge>");
    assert_eq!(
        result,
        vec!["hkArrayBase<hkaiDirectedGraphExplicitCost::Edge>"]
    );
}

#[test]
fn test_type_115() {
    let result = parse_type("struct __cppobj hkRefPtr<hkaiSilhouettePriorityController const >");
    assert_eq!(
        result,
        vec!["hkRefPtr<hkaiSilhouettePriorityController const >"]
    );
}

#[test]
fn test_type_116() {
    let result = parse_type("struct __cppobj hkndWorld::PostStepSignal : hkSignal1<hkndWorld *>");
    assert_eq!(result, vec!["hkndWorld", "PostStepSignal"]);
}

#[test]
fn test_type_117() {
    let result = parse_type("struct /*VFT*/ CBaseNetworkComponentManager_vtbl");
    assert_eq!(result, vec!["CBaseNetworkComponentManager_vtbl"]);
}

#[test]
fn test_type_118() {
    let result = parse_type("struct __cppobj ExtractValueHelper<VehicleValueCache,3031391367>");
    assert_eq!(
        result,
        vec!["ExtractValueHelper<VehicleValueCache,3031391367>"]
    );
}

#[test]
fn test_type_119() {
    let result = parse_type("struct SLandSteering");
    assert_eq!(result, vec!["SLandSteering"]);
}

#[test]
fn test_type_120() {
    let result = parse_type("struct __cppobj hkArrayBase<hkcdPlanarGeometryPrimitives::Plane>");
    assert_eq!(
        result,
        vec!["hkArrayBase<hkcdPlanarGeometryPrimitives::Plane>"]
    );
}

#[test]
fn test_type_121() {
    let result = parse_type("struct SMissileWeaponTuning");
    assert_eq!(result, vec!["SMissileWeaponTuning"]);
}

#[test]
fn test_type_122() {
    let result =
        parse_type("struct __cppobj std::_Pair_base<unsigned int const ,SPatchCacheEntry>");
    assert_eq!(
        result,
        vec!["std", "_Pair_base<unsigned int const ,SPatchCacheEntry>"]
    );
}

#[test]
fn test_type_123() {
    let result = parse_type("struct /*VFT*/ NUIManager::ScaleFormText_vtbl");
    assert_eq!(result, vec!["NUIManager", "ScaleFormText_vtbl"]);
}

#[test]
fn test_type_124() {
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
fn test_type_125() {
    let result = parse_type(
        "struct __cppobj std::vector<NGraphicsEngine::SScopeIndexer> : std::_Vector_val<NGraphicsEngine::SScopeIndexer>",
    );
    assert_eq!(
        result,
        vec!["std", "vector<NGraphicsEngine::SScopeIndexer>"]
    );
}

#[test]
fn test_type_126() {
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
fn test_type_127() {
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
fn test_type_128() {
    let result = parse_type("struct __cppobj std::_Allocator_base<CClimateZone::STextureZone>");
    assert_eq!(
        result,
        vec!["std", "_Allocator_base<CClimateZone::STextureZone>"]
    );
}

#[test]
fn test_type_129() {
    let result = parse_type("struct Input::SDeviceType");
    assert_eq!(result, vec!["Input", "SDeviceType"]);
}

#[test]
fn test_type_130() {
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
fn test_type_131() {
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
fn test_type_132() {
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
fn test_type_133() {
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
fn test_type_134() {
    let result = parse_type(
        "struct __cppobj __declspec(align(8)) std::_Vector_val<STrafficZone *> : std::_Container_base0",
    );
    assert_eq!(result, vec!["std", "_Vector_val<STrafficZone *>"]);
}

#[test]
fn test_type_135() {
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
fn test_type_136() {
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
fn test_type_137() {
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
fn test_type_138() {
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
fn test_type_139() {
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
fn test_type_140() {
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
fn test_type_141() {
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
fn test_type_142() {
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
fn test_type_143() {
    let result = parse_type("struct __cppobj std::_Pair_base<CHashString,STaskGroupInfo>");
    assert_eq!(
        result,
        vec!["std", "_Pair_base<CHashString,STaskGroupInfo>"]
    );
}

#[test]
fn test_type_144() {
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
fn test_type_145() {
    let result = parse_type("struct __cppobj std::tr1::_Callable_base<<lambda16>,0>");
    assert_eq!(result, vec!["std", "tr1", "_Callable_base<<lambda16>,0>"]);
}

#[test]
fn test_type_146() {
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
fn test_type_147() {
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
fn test_type_148() {
    let result = parse_type(
        "struct __cppobj std::_Vector_const_iterator<std::_Vector_val<void *> > : std::_Iterator012<std::random_access_iterator_tag,void *,__int64,void * const *,void * const &,std::_Iterator_base0>",
    );
    assert_eq!(
        result,
        vec!["std", "_Vector_const_iterator<std::_Vector_val<void *> >"]
    );
}

#[test]
fn test_type_149() {
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
fn test_type_150() {
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
fn test_type_151() {
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
fn test_type_152() {
    let result = parse_type("struct __cppobj INetworkConnection : IDispatch");
    assert_eq!(result, vec!["INetworkConnection"]);
}

#[test]
fn test_type_153() {
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
fn test_type_154() {
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
fn test_type_155() {
    let result = parse_type("struct __cppobj hkRefNew<hkaiNavMesh const >");
    assert_eq!(result, vec!["hkRefNew<hkaiNavMesh const >"]);
}

#[test]
fn test_type_156() {
    let result = parse_type("struct hkMxMaskd<3>");
    assert_eq!(result, vec!["hkMxMaskd<3>"]);
}

#[test]
fn test_type_157() {
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
fn test_type_158() {
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
fn test_type_159() {
    let result = parse_type("struct __cppobj Scaleform::ThreadCheckBase");
    assert_eq!(result, vec!["Scaleform", "ThreadCheckBase"]);
}

#[test]
fn test_type_160() {
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
fn test_type_161() {
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
fn test_type_162() {
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
fn test_type_163() {
    let result = parse_type(
        "struct __cppobj __declspec(align(4)) Scaleform::GFx::TouchEventId : Scaleform::GFx::EventId",
    );
    assert_eq!(result, vec!["Scaleform", "GFx", "TouchEventId"]);
}

#[test]
fn test_type_164() {
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
fn test_type_165() {
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
fn test_type_166() {
    let result = parse_type("struct /*VFT*/ Scaleform::GFx::AS3::IMEManager_vtbl");
    assert_eq!(result, vec!["Scaleform", "GFx", "AS3", "IMEManager_vtbl"]);
}

#[test]
fn test_type_167() {
    let result = parse_type("struct /*VFT*/ Scaleform::GFx::AS3Support_vtbl");
    assert_eq!(result, vec!["Scaleform", "GFx", "AS3Support_vtbl"]);
}

#[test]
fn test_type_168() {
    let result = parse_type(
        "struct __cppobj Scaleform::Render::StateData::Interface_Value : Scaleform::Render::StateData::Interface",
    );
    assert_eq!(
        result,
        vec!["Scaleform", "Render", "StateData", "Interface_Value"]
    );
}

#[test]
fn test_type_169() {
    let result = parse_type("struct /*VFT*/ Scaleform::Render::TGA::FileWriter_vtbl");
    assert_eq!(
        result,
        vec!["Scaleform", "Render", "TGA", "FileWriter_vtbl"]
    );
}

#[test]
fn test_type_170() {
    let result = parse_type("struct __cppobj Scaleform::Render::Text::TextFormat::HashFunctor");
    assert_eq!(
        result,
        vec!["Scaleform", "Render", "Text", "TextFormat", "HashFunctor"]
    );
}

#[test]
fn test_type_171() {
    let result = parse_type("struct __cppobj __declspec(align(8)) CHeatVocals : CVocalsBase");
    assert_eq!(result, vec!["CHeatVocals"]);
}

#[test]
fn test_type_172() {
    let result = parse_type("struct __cppobj hkMonitorStreamFrameInfo");
    assert_eq!(result, vec!["hkMonitorStreamFrameInfo"]);
}

#[test]
fn test_type_173() {
    let result = parse_type("struct __cppobj NGameSoundLibrary::SCirclePreCalc");
    assert_eq!(result, vec!["NGameSoundLibrary", "SCirclePreCalc"]);
}

#[test]
fn test_type_174() {
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
fn test_type_175() {
    let result = parse_type("struct /*VFT*/ hkSignal2<hknpWorld *,hknpConstraint *>::Slot_vtbl");
    assert_eq!(
        result,
        vec!["hkSignal2<hknpWorld *,hknpConstraint *>", "Slot_vtbl"]
    );
}

#[test]
fn test_type_176() {
    let result = parse_type("struct /*VFT*/ hkcdConvexCellsCollection_vtbl");
    assert_eq!(result, vec!["hkcdConvexCellsCollection_vtbl"]);
}

#[test]
fn test_type_177() {
    let result = parse_type("struct __cppobj std::_Allocator_base<CRoadMeshManager::SRoad>");
    assert_eq!(
        result,
        vec!["std", "_Allocator_base<CRoadMeshManager::SRoad>"]
    );
}

#[test]
fn test_type_178() {
    let result = parse_type("struct __cppobj FollowLinkHelper<VehicleValueCache,7>");
    assert_eq!(result, vec!["FollowLinkHelper<VehicleValueCache,7>"]);
}

#[test]
fn test_type_179() {
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
fn test_type_180() {
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
fn test_type_181() {
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
fn test_type_182() {
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
fn test_type_183() {
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
fn test_type_184() {
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
fn test_type_185() {
    let result = parse_type("struct __cppobj <lambda340>");
    assert_eq!(result, vec!["<lambda340>"]);
}

#[test]
fn test_type_186() {
    let result = parse_type("struct __cppobj boost::detail::addressof_impl<<lambda206> >");
    assert_eq!(
        result,
        vec!["boost", "detail", "addressof_impl<<lambda206> >"]
    );
}

#[test]
fn test_type_187() {
    let result = parse_type("struct __cppobj boost::detail::addr_impl_ref<<lambda316> >");
    assert_eq!(
        result,
        vec!["boost", "detail", "addr_impl_ref<<lambda316> >"]
    );
}

#[test]
fn test_type_188() {
    let result = parse_type("struct __cppobj <lambda351>");
    assert_eq!(result, vec!["<lambda351>"]);
}

#[test]
fn test_type_189() {
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
fn test_type_190() {
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
fn test_type_191() {
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
fn test_type_192() {
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
fn test_type_193() {
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
fn test_type_194() {
    let result = parse_type("struct __cppobj boost::detail::addressof_impl<<lambda221> >");
    assert_eq!(
        result,
        vec!["boost", "detail", "addressof_impl<<lambda221> >"]
    );
}

#[test]
fn test_type_195() {
    let result = parse_type("struct __cppobj boost::detail::addr_impl_ref<<lambda133> >");
    assert_eq!(
        result,
        vec!["boost", "detail", "addr_impl_ref<<lambda133> >"]
    );
}

#[test]
fn test_type_196() {
    let result = parse_type("struct __cppobj boost::detail::core_typeid_<<lambda243> >");
    assert_eq!(
        result,
        vec!["boost", "detail", "core_typeid_<<lambda243> >"]
    );
}

#[test]
fn test_type_197() {
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
fn test_type_198() {
    let result = parse_type("struct /*VFT*/ CAiAreaOfOperationsEntity_vtbl");
    assert_eq!(result, vec!["CAiAreaOfOperationsEntity_vtbl"]);
}

#[test]
fn test_type_199() {
    let result =
        parse_type("struct __cppobj hknpDestructionBreakOffModifier::ContactEvent : hkCommand");
    assert_eq!(
        result,
        vec!["hknpDestructionBreakOffModifier", "ContactEvent"]
    );
}

#[test]
fn test_type_200() {
    let result = parse_type("struct __cppobj ValueTypeTrait<2728888050>");
    assert_eq!(result, vec!["ValueTypeTrait<2728888050>"]);
}

#[test]
fn test_type_201() {
    let result = parse_type("struct $_TypeDescriptor$_extraBytes_40");
    assert_eq!(result, vec!["$_TypeDescriptor$_extraBytes_40"]);
}

#[test]
fn test_type_202() {
    let result = parse_type("struct __cppobj hkRefLoan<hkaiAvoidanceProperties const >");
    assert_eq!(result, vec!["hkRefLoan<hkaiAvoidanceProperties const >"]);
}

#[test]
fn test_type_203() {
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
fn test_type_204() {
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
fn test_type_205() {
    let result = parse_type("struct __cppobj hkSimdReal_AdvancedInterface::unrollf_loadH<0>");
    assert_eq!(
        result,
        vec!["hkSimdReal_AdvancedInterface", "unrollf_loadH<0>"]
    );
}

#[test]
fn test_type_206() {
    let result = parse_type("struct __cppobj ExtractValueHelper<VehicleValueCache,423992487>");
    assert_eq!(
        result,
        vec!["ExtractValueHelper<VehicleValueCache,423992487>"]
    );
}

#[test]
fn test_type_207() {
    let result = parse_type("struct SPartPhysicsMappings");
    assert_eq!(result, vec!["SPartPhysicsMappings"]);
}

#[test]
fn test_type_208() {
    let result = parse_type(
        "struct __cppobj TIterableItemPool<CLightInstantiator,16>::iterator : TIterableItemPool<CLightInstantiator,16>::const_iterator",
    );
    assert_eq!(
        result,
        vec!["TIterableItemPool<CLightInstantiator,16>", "iterator"]
    );
}

#[test]
fn test_type_209() {
    let result = parse_type("struct __declspec(align(8)) STerrainSystem::ArrayShaderRules");
    assert_eq!(result, vec!["STerrainSystem", "ArrayShaderRules"]);
}

#[test]
fn test_type_210() {
    let result = parse_type(
        "struct __cppobj std::allocator<CHUDUI::SWireData> : std::_Allocator_base<CHUDUI::SWireData>",
    );
    assert_eq!(result, vec!["std", "allocator<CHUDUI::SWireData>"]);
}

#[test]
fn test_type_211() {
    let result = parse_type("struct /*VFT*/ CCommBragsFeatsUI_vtbl");
    assert_eq!(result, vec!["CCommBragsFeatsUI_vtbl"]);
}

#[test]
fn test_type_212() {
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
fn test_type_213() {
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
fn test_type_214() {
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
fn test_type_215() {
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
fn test_type_216() {
    let result = parse_type(
        "struct __cppobj std::tr1::_Callable_obj<<lambda89>,0> : std::tr1::_Callable_base<<lambda89>,0>",
    );
    assert_eq!(result, vec!["std", "tr1", "_Callable_obj<<lambda89>,0>"]);
}

#[test]
fn test_type_217() {
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
fn test_type_218() {
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
fn test_type_219() {
    let result = parse_type("struct __cppobj Scaleform::Pickable<Scaleform::GFx::AS3::Class>");
    assert_eq!(
        result,
        vec!["Scaleform", "Pickable<Scaleform::GFx::AS3::Class>"]
    );
}

#[test]
fn test_type_220() {
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
fn test_type_221() {
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
fn test_type_222() {
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
fn test_type_223() {
    let result = parse_type(
        "struct __cppobj __declspec(align(8)) NAr::detail::CLoadSchemaVisitor<NAr::CBlobStream>",
    );
    assert_eq!(
        result,
        vec!["NAr", "detail", "CLoadSchemaVisitor<NAr::CBlobStream>"]
    );
}

#[test]
fn test_type_224() {
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
fn test_type_225() {
    let result = parse_type("struct __cppobj std::_Pair_base<short,unsigned short>");
    assert_eq!(result, vec!["std", "_Pair_base<short,unsigned short>"]);
}

#[test]
fn test_type_226() {
    let result = parse_type("struct /*VFT*/ NSaveLoad::CRemoteDataRef<int>_vtbl");
    assert_eq!(result, vec!["NSaveLoad", "CRemoteDataRef<int>_vtbl"]);
}

#[test]
fn test_type_227() {
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
fn test_type_228() {
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
fn test_type_229() {
    let result = parse_type(
        "struct __cppobj boost::rational<int> : boost::less_than_comparable<boost::rational<int>,boost::equality_comparable<boost::rational<int>,boost::less_than_comparable2<boost::rational<int>,int,boost::equality_comparable2<boost::rational<int>,int,boost::addable<boost::rational<int>,boost::subtractable<boost::rational<int>,boost::multipliable<boost::rational<int>,boost::dividable<boost::rational<int>,boost::addable2<boost::rational<int>,int,boost::subtractable2<boost::rational<int>,int,boost::subtractable2_left<boost::rational<int>,int,boost::multipliable2<boost::rational<int>,int,boost::dividable2<boost::rational<int>,int,boost::dividable2_left<boost::rational<int>,int,boost::incrementable<boost::rational<int>,boost::decrementable<boost::rational<int>,boost::detail::empty_base<boost::rational<int> > > > > > > > > >,boost::detail::empty_base<boost::rational<int> >,boost::detail::true_t>,boost::detail::empty_base<boost::rational<int> >,boost::detail::true_t>,boost::detail::empty_base<boost::rational<int> >,boost::detail::true_t>,boost::detail::empty_base<boost::rational<int> >,boost::detail::true_t> > >,boost::detail::empty_base<boost::rational<int> >,boost::detail::true_t>,boost::detail::empty_base<boost::rational<int> >,boost::detail::true_t>",
    );
    assert_eq!(result, vec!["boost", "rational<int>"]);
}

#[test]
fn test_type_230() {
    let result = parse_type(
        "struct __cppobj NGraphicsEngine::COccluderCollectionManager : Base::CSingle<NGraphicsEngine::COccluderCollectionManager>",
    );
    assert_eq!(
        result,
        vec!["NGraphicsEngine", "COccluderCollectionManager"]
    );
}

#[test]
fn test_type_231() {
    let result = parse_type("struct __cppobj hkFreeList");
    assert_eq!(result, vec!["hkFreeList"]);
}

#[test]
fn test_type_232() {
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
fn test_type_233() {
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
fn test_type_234() {
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
fn test_type_235() {
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
fn test_type_236() {
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
fn test_type_237() {
    let result = parse_type("struct SOfferItem");
    assert_eq!(result, vec!["SOfferItem"]);
}

#[test]
fn test_type_238() {
    let result = parse_type("struct SGear");
    assert_eq!(result, vec!["SGear"]);
}

#[test]
fn test_type_239() {
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
fn test_type_240() {
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
fn test_type_241() {
    let result = parse_type("struct __cppobj boost::detail::addr_impl_ref<float>");
    assert_eq!(result, vec!["boost", "detail", "addr_impl_ref<float>"]);
}

#[test]
fn test_type_242() {
    let result = parse_type("struct __cppobj NOnline::<lambda245>");
    assert_eq!(result, vec!["NOnline", "<lambda245>"]);
}

#[test]
fn test_type_243() {
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
fn test_type_244() {
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
fn test_type_245() {
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
fn test_type_246() {
    let result = parse_type("struct __cppobj boost::uuids::detail::seed_rng");
    assert_eq!(result, vec!["boost", "uuids", "detail", "seed_rng"]);
}

#[test]
fn test_type_247() {
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
fn test_type_248() {
    let result = parse_type("struct __cppobj hkArrayBase<hkgpConvexHull *>");
    assert_eq!(result, vec!["hkArrayBase<hkgpConvexHull *>"]);
}

#[test]
fn test_type_249() {
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
fn test_type_250() {
    let result = parse_type("struct __cppobj boost::detail::sp_ms_deleter<CFriend>");
    assert_eq!(result, vec!["boost", "detail", "sp_ms_deleter<CFriend>"]);
}

#[test]
fn test_type_251() {
    let result = parse_type("struct __cppobj std::tr1::_Callable_base<<lambda20>,0>");
    assert_eq!(result, vec!["std", "tr1", "_Callable_base<<lambda20>,0>"]);
}

#[test]
fn test_type_252() {
    let result = parse_type("struct __cppobj CScrapyardROMNetworkComponent : CROMNetworkComponent");
    assert_eq!(result, vec!["CScrapyardROMNetworkComponent"]);
}

#[test]
fn test_type_253() {
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
fn test_type_254() {
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
fn test_type_255() {
    let result = parse_type("struct /*VFT*/ CGraphEntryOccupiedByAuthorityCondition_vtbl");
    assert_eq!(result, vec!["CGraphEntryOccupiedByAuthorityCondition_vtbl"]);
}

#[test]
fn test_type_256() {
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
fn test_type_257() {
    let result = parse_type(
        "struct __cppobj __declspec(align(8)) CGraphEntryIsFacingNodeCondition : CInteractionGraphEntryAFSMC",
    );
    assert_eq!(result, vec!["CGraphEntryIsFacingNodeCondition"]);
}

#[test]
fn test_type_258() {
    let result = parse_type("struct __cppobj NGSONodes::CNodeCounterHandler");
    assert_eq!(result, vec!["NGSONodes", "CNodeCounterHandler"]);
}

#[test]
fn test_type_259() {
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
fn test_type_260() {
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
fn test_type_261() {
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
fn test_type_262() {
    let result = parse_type("struct __cppobj CAIRoadObstacle : CGameObject");
    assert_eq!(result, vec!["CAIRoadObstacle"]);
}

#[test]
fn test_type_263() {
    let result = parse_type(
        "struct __cppobj CGameObjectCreator<CDynamicNavMeshCutter> : IGameObjectCreator",
    );
    assert_eq!(result, vec!["CGameObjectCreator<CDynamicNavMeshCutter>"]);
}

#[test]
fn test_type_264() {
    let result = parse_type("struct /*VFT*/ CActionTokenManager_vtbl");
    assert_eq!(result, vec!["CActionTokenManager_vtbl"]);
}

#[test]
fn test_type_265() {
    let result =
        parse_type("struct __cppobj CGameObjectCreator<CSniperWeapon>::SGameObjectDestroyer");
    assert_eq!(
        result,
        vec!["CGameObjectCreator<CSniperWeapon>", "SGameObjectDestroyer"]
    );
}

#[test]
fn test_type_266() {
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
fn test_type_267() {
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
fn test_type_268() {
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
fn test_type_269() {
    let result = parse_type("struct /*VFT*/ CGameObjectCreator<CWingsuitRing>_vtbl");
    assert_eq!(result, vec!["CGameObjectCreator<CWingsuitRing>_vtbl"]);
}

#[test]
fn test_type_270() {
    let result = parse_type("struct /*VFT*/ CAppSystemCreator<CTargetSystem>_vtbl");
    assert_eq!(result, vec!["CAppSystemCreator<CTargetSystem>_vtbl"]);
}

#[test]
fn test_type_271() {
    let result = parse_type("struct /*VFT*/ CGameObjectCreator<CSungunWeapon>_vtbl");
    assert_eq!(result, vec!["CGameObjectCreator<CSungunWeapon>_vtbl"]);
}

#[test]
fn test_type_272() {
    let result = parse_type("struct __cppobj boost::shared_ptr<CGrenade>");
    assert_eq!(result, vec!["boost", "shared_ptr<CGrenade>"]);
}

#[test]
fn test_type_273() {
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
fn test_type_274() {
    let result = parse_type("struct MemoryBlockStat");
    assert_eq!(result, vec!["MemoryBlockStat"]);
}

#[test]
fn test_type_275() {
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
fn test_type_276() {
    let result = parse_type("struct __cppobj hkRefLoan<hkaiStreamingCollection>");
    assert_eq!(result, vec!["hkRefLoan<hkaiStreamingCollection>"]);
}

#[test]
fn test_type_277() {
    let result = parse_type("struct __cppobj NInputTasks::NRelease::SProperties");
    assert_eq!(result, vec!["NInputTasks", "NRelease", "SProperties"]);
}

#[test]
fn test_type_278() {
    let result = parse_type("struct /*VFT*/ CHorizontalSpeedCondition_vtbl");
    assert_eq!(result, vec!["CHorizontalSpeedCondition_vtbl"]);
}

#[test]
fn test_type_279() {
    let result = parse_type("struct __cppobj CIsPlayerNonCombat : NAnimationSystem::CCondition");
    assert_eq!(result, vec!["CIsPlayerNonCombat"]);
}

#[test]
fn test_type_280() {
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
fn test_type_281() {
    let result =
        parse_type("struct __unaligned __declspec(align(1)) $_TypeDescriptor$_extraBytes_161");
    assert_eq!(result, vec!["$_TypeDescriptor$_extraBytes_161"]);
}

#[test]
fn test_type_282() {
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
fn test_type_283() {
    let result = parse_type("struct /*VFT*/ hkaiUserEdgePairArray_vtbl");
    assert_eq!(result, vec!["hkaiUserEdgePairArray_vtbl"]);
}

#[test]
fn test_type_284() {
    let result = parse_type("struct __cppobj hkArrayBase<hkMonitorStreamStringMap::StringMap>");
    assert_eq!(
        result,
        vec!["hkArrayBase<hkMonitorStreamStringMap::StringMap>"]
    );
}

#[test]
fn test_type_285() {
    let result = parse_type("struct __cppobj CFootpathFilterFtor : IRoadFilterFtor");
    assert_eq!(result, vec!["CFootpathFilterFtor"]);
}

#[test]
fn test_type_286() {
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
fn test_type_287() {
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
fn test_type_288() {
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
fn test_type_289() {
    let result = parse_type(
        "struct __cppobj boost::fusion::vector1<enum dynamo::ast::op_token &> : boost::fusion::vector_data1<enum dynamo::ast::op_token &>, boost::fusion::sequence_base<boost::fusion::vector1<enum dynamo::ast::op_token &> >",
    );
    assert_eq!(
        result,
        vec!["boost", "fusion", "vector1<enum dynamo::ast::op_token &>"]
    );
}

#[test]
fn test_type_290() {
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
fn test_type_291() {
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
fn test_type_292() {
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
fn test_type_293() {
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
fn test_type_294() {
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
fn test_type_295() {
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
fn test_type_296() {
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
fn test_type_297() {
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
fn test_type_298() {
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
fn test_type_299() {
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
fn test_type_300() {
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
fn test_type_301() {
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
fn test_type_302() {
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
fn test_type_303() {
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
fn test_type_304() {
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
fn test_type_305() {
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
fn test_type_306() {
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
fn test_type_307() {
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
fn test_type_308() {
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
fn test_type_309() {
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
fn test_type_310() {
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
fn test_type_311() {
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
fn test_type_312() {
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
fn test_type_313() {
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
fn test_type_314() {
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
fn test_type_315() {
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
fn test_type_316() {
    let result = parse_type("struct __cppobj erase_swap_multiple_Test : testing::Test");
    assert_eq!(result, vec!["erase_swap_multiple_Test"]);
}

#[test]
fn test_type_317() {
    let result = parse_type("union simd_extract_fp::__l2::<unnamed_type_u>");
    assert_eq!(result, vec!["simd_extract_fp", "__l2", "<unnamed_type_u>"]);
}

#[test]
fn test_type_318() {
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
fn test_type_319() {
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
fn test_type_320() {
    let result = parse_type(
        "struct __cppobj __declspec(align(8)) CCameraConditionRotation : CCameraCondition",
    );
    assert_eq!(result, vec!["CCameraConditionRotation"]);
}

#[test]
fn test_type_321() {
    let result = parse_type("struct __cppobj index_ptr<NAnimationSystem::CSubtractRotationsBox>");
    assert_eq!(
        result,
        vec!["index_ptr<NAnimationSystem::CSubtractRotationsBox>"]
    );
}

#[test]
fn test_type_322() {
    let result = parse_type("struct /*VFT*/ NAnimationSystem::CVectorToNumberBox_vtbl");
    assert_eq!(result, vec!["NAnimationSystem", "CVectorToNumberBox_vtbl"]);
}

#[test]
fn test_type_323() {
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
fn test_type_324() {
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
fn test_type_325() {
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
fn test_type_326() {
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
fn test_type_327() {
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
fn test_type_328() {
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
fn test_type_329() {
    let result = parse_type("struct _DLLVERSIONINFO");
    assert_eq!(result, vec!["_DLLVERSIONINFO"]);
}

#[test]
fn test_type_330() {
    let result = parse_type("struct __cppobj __declspec(align(8)) SPatchSystemUpdateDesc");
    assert_eq!(result, vec!["SPatchSystemUpdateDesc"]);
}

#[test]
fn test_type_331() {
    let result = parse_type("struct __cppobj CRawBitmap<_PixelFloatRGBA>");
    assert_eq!(result, vec!["CRawBitmap<_PixelFloatRGBA>"]);
}

#[test]
fn test_type_332() {
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
fn test_type_333() {
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
fn test_type_334() {
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
fn test_type_335() {
    let result = parse_type("struct __declspec(align(8)) hkSolverAllocator::Element");
    assert_eq!(result, vec!["hkSolverAllocator", "Element"]);
}

#[test]
fn test_type_336() {
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
fn test_type_337() {
    let result =
        parse_type("struct __cppobj __declspec(align(4)) hknpAddBodyCommand : hknpApiCommand");
    assert_eq!(result, vec!["hknpAddBodyCommand"]);
}

#[test]
fn test_type_338() {
    let result = parse_type("struct __cppobj NGraphicsEngine::CompareTypeThenSortId");
    assert_eq!(result, vec!["NGraphicsEngine", "CompareTypeThenSortId"]);
}

#[test]
fn test_type_339() {
    let result = parse_type("struct __cppobj NGraphicsEngine::CRenderBlockFont::STextBuffer");
    assert_eq!(
        result,
        vec!["NGraphicsEngine", "CRenderBlockFont", "STextBuffer"]
    );
}

#[test]
fn test_type_340() {
    let result = parse_type("struct __cppobj FileInfo");
    assert_eq!(result, vec!["FileInfo"]);
}

#[test]
fn test_type_341() {
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
fn test_type_342() {
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
fn test_type_343() {
    let result = parse_type("struct _STARTUPINFOW");
    assert_eq!(result, vec!["_STARTUPINFOW"]);
}

#[test]
fn test_type_344() {
    let result = parse_type("struct __cppobj int3");
    assert_eq!(result, vec!["int3"]);
}

#[test]
fn test_type_345() {
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
fn test_type_346() {
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
fn test_type_347() {
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
fn test_type_348() {
    let result = parse_type("struct __cppobj SVfsHttpArchiveProviderInstance");
    assert_eq!(result, vec!["SVfsHttpArchiveProviderInstance"]);
}

#[test]
fn test_type_349() {
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
fn test_type_350() {
    let result =
        parse_type("struct /*VFT*/ hkCrcStreamWriter<unsigned __int64,-3932672073523589310>_vtbl");
    assert_eq!(
        result,
        vec!["hkCrcStreamWriter<unsigned __int64,-3932672073523589310>_vtbl"]
    );
}

#[test]
fn test_type_351() {
    let result = parse_type("struct __cppobj hkArrayBase<hkMeshBody *>");
    assert_eq!(result, vec!["hkArrayBase<hkMeshBody *>"]);
}

#[test]
fn test_type_352() {
    let result = parse_type(
        "struct __cppobj hkArray<hkMeshShape *,hkContainerHeapAllocator> : hkArrayBase<hkMeshShape *>",
    );
    assert_eq!(
        result,
        vec!["hkArray<hkMeshShape *,hkContainerHeapAllocator>"]
    );
}

#[test]
fn test_type_353() {
    let result = parse_type(
        "struct __cppobj hkArray<hkxMaterial *,hkContainerHeapAllocator> : hkArrayBase<hkxMaterial *>",
    );
    assert_eq!(
        result,
        vec!["hkArray<hkxMaterial *,hkContainerHeapAllocator>"]
    );
}

#[test]
fn test_type_354() {
    let result = parse_type("struct __cppobj hkArrayBase<hkDataClassDict::Enum *>");
    assert_eq!(result, vec!["hkArrayBase<hkDataClassDict::Enum *>"]);
}

#[test]
fn test_type_355() {
    let result = parse_type(
        "struct __cppobj __declspec(align(8)) hkLocalArray<unsigned char> : hkArray<unsigned char,hkContainerHeapAllocator>",
    );
    assert_eq!(result, vec!["hkLocalArray<unsigned char>"]);
}

#[test]
fn test_type_356() {
    let result = parse_type("struct __cppobj hkRefLoan<hkMeshBody>");
    assert_eq!(result, vec!["hkRefLoan<hkMeshBody>"]);
}

#[test]
fn test_type_357() {
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
fn test_type_358() {
    let result = parse_type("struct __cppobj hkMxVectorf_Implementation::lengthH<1,3,2,1>");
    assert_eq!(
        result,
        vec!["hkMxVectorf_Implementation", "lengthH<1,3,2,1>"]
    );
}

#[test]
fn test_type_359() {
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
fn test_type_360() {
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
fn test_type_361() {
    let result = parse_type("struct __cppobj hkgpMeshInternals::Ring::Segment");
    assert_eq!(result, vec!["hkgpMeshInternals", "Ring", "Segment"]);
}

#[test]
fn test_type_362() {
    let result = parse_type("struct /*VFT*/ hkcdSimdTree::PairCollector_vtbl");
    assert_eq!(result, vec!["hkcdSimdTree", "PairCollector_vtbl"]);
}

#[test]
fn test_type_363() {
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
fn test_type_364() {
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
fn test_type_365() {
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
fn test_type_366() {
    let result = parse_type("struct __cppobj __vc_attributes::odlAttribute");
    assert_eq!(result, vec!["__vc_attributes", "odlAttribute"]);
}

#[test]
fn test_type_367() {
    let result = parse_type("struct __cppobj hkCompileError::MX_SUBVECTORf_INDEX_OUT_OF_RANGE<1>");
    assert_eq!(
        result,
        vec!["hkCompileError", "MX_SUBVECTORf_INDEX_OUT_OF_RANGE<1>"]
    );
}

#[test]
fn test_type_368() {
    let result = parse_type(
        "struct __cppobj hkpJacobian1dLinearLimitsSchema : hkpJacobianSchemaInfo::LinearLimits1D",
    );
    assert_eq!(result, vec!["hkpJacobian1dLinearLimitsSchema"]);
}

#[test]
fn test_type_369() {
    let result =
        parse_type("struct __cppobj __declspec(align(8)) StreamContactSolver::WriterHelper<0>");
    assert_eq!(result, vec!["StreamContactSolver", "WriterHelper<0>"]);
}

#[test]
fn test_type_370() {
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
fn test_type_371() {
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
fn test_type_372() {
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
fn test_type_373() {
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
fn test_type_374() {
    let result = parse_type(
        "struct __cppobj hkArrayBase<hkcdDynamicTree::Tree<hkcdDynamicTree::DynamicStorage16>::SAHBin>",
    );
    assert_eq!(
        result,
        vec!["hkArrayBase<hkcdDynamicTree::Tree<hkcdDynamicTree::DynamicStorage16>::SAHBin>"]
    );
}

#[test]
fn test_type_375() {
    let result = parse_type("struct _REMOTE_NAME_INFOW");
    assert_eq!(result, vec!["_REMOTE_NAME_INFOW"]);
}

#[test]
fn test_type_376() {
    let result = parse_type("struct tagSIZE");
    assert_eq!(result, vec!["tagSIZE"]);
}

#[test]
fn test_type_377() {
    let result = parse_type("struct __cppobj hkTrait::RemoveConst<hknpMotionId>");
    assert_eq!(result, vec!["hkTrait", "RemoveConst<hknpMotionId>"]);
}

#[test]
fn test_type_378() {
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
fn test_type_379() {
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
fn test_type_380() {
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
fn test_type_381() {
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
fn test_type_382() {
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
fn test_type_383() {
    let result = parse_type("struct __cppobj hknpCachedVertexReader");
    assert_eq!(result, vec!["hknpCachedVertexReader"]);
}

#[test]
fn test_type_384() {
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
fn test_type_385() {
    let result = parse_type(
        "struct __cppobj hkArray<hkRefPtr<hkaAnimation>,hkContainerHeapAllocator> : hkArrayBase<hkRefPtr<hkaAnimation> >",
    );
    assert_eq!(
        result,
        vec!["hkArray<hkRefPtr<hkaAnimation>,hkContainerHeapAllocator>"]
    );
}

#[test]
fn test_type_386() {
    let result = parse_type("struct hkaiAdaptiveRanger_DefaultStruct");
    assert_eq!(result, vec!["hkaiAdaptiveRanger_DefaultStruct"]);
}

#[test]
fn test_type_387() {
    let result = parse_type("struct hkaiSplitGenerationUtilsSettings_DefaultStruct");
    assert_eq!(
        result,
        vec!["hkaiSplitGenerationUtilsSettings_DefaultStruct"]
    );
}

#[test]
fn test_type_388() {
    let result =
        parse_type("struct __cppobj __declspec(align(8)) hkFixedArray<hk1AxisSweep::AabbInt>");
    assert_eq!(result, vec!["hkFixedArray<hk1AxisSweep::AabbInt>"]);
}

#[test]
fn test_type_389() {
    let result = parse_type("struct /*VFT*/ hkaiSilhouetteRecorder::GraphUnloadedEvent_vtbl");
    assert_eq!(
        result,
        vec!["hkaiSilhouetteRecorder", "GraphUnloadedEvent_vtbl"]
    );
}

#[test]
fn test_type_390() {
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
fn test_type_391() {
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
fn test_type_392() {
    let result =
        parse_type("struct __cppobj GeometryCastSegmentQuery : hkcdAabbTreeQueries::AabbCollector");
    assert_eq!(result, vec!["GeometryCastSegmentQuery"]);
}

#[test]
fn test_type_393() {
    let result =
        parse_type("struct __unaligned __declspec(align(1)) $_TypeDescriptor$_extraBytes_247");
    assert_eq!(result, vec!["$_TypeDescriptor$_extraBytes_247"]);
}

#[test]
fn test_type_394() {
    let result = parse_type(
        "struct __cppobj hkndBodyTimeoutRuntime::BreakableBodyTimeoutSignal : hkSignal2<hkndWorld *,hkHandle<unsigned short,65535,hkndBodyUniqueId> >",
    );
    assert_eq!(
        result,
        vec!["hkndBodyTimeoutRuntime", "BreakableBodyTimeoutSignal"]
    );
}

#[test]
fn test_type_395() {
    let result = parse_type("union getVtablehkndShockWaveAction::__l2::<unnamed_type_u>");
    assert_eq!(
        result,
        vec!["getVtablehkndShockWaveAction", "__l2", "<unnamed_type_u>"]
    );
}

#[test]
fn test_type_396() {
    let result = parse_type(
        "struct __cppobj __declspec(align(16)) hkndIntegrityAnalyzerRuntimeImpl::DisableConns",
    );
    assert_eq!(
        result,
        vec!["hkndIntegrityAnalyzerRuntimeImpl", "DisableConns"]
    );
}

#[test]
fn test_type_397() {
    let result = parse_type("struct __cppobj hkndSolidGeometryToShapeConverter");
    assert_eq!(result, vec!["hkndSolidGeometryToShapeConverter"]);
}

#[test]
fn test_type_398() {
    let result = parse_type("struct $_TypeDescriptor$_extraBytes_384");
    assert_eq!(result, vec!["$_TypeDescriptor$_extraBytes_384"]);
}

#[test]
fn test_type_399() {
    let result = parse_type("struct __cppobj hkRefNew<hkndFractureLineMapImpl::MeshSection>");
    assert_eq!(
        result,
        vec!["hkRefNew<hkndFractureLineMapImpl::MeshSection>"]
    );
}

#[test]
fn test_type_400() {
    let result = parse_type("struct /*VFT*/ hkdWoodFracture_vtbl");
    assert_eq!(result, vec!["hkdWoodFracture_vtbl"]);
}

#[test]
fn test_type_401() {
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
fn test_type_402() {
    let result = parse_type("struct __cppobj hkpCollisionFilterList : hkpCollisionFilter");
    assert_eq!(result, vec!["hkpCollisionFilterList"]);
}

#[test]
fn test_type_403() {
    let result = parse_type("struct __cppobj hkRefPtr<hkpBreakableMultiMaterial::InverseMapping>");
    assert_eq!(
        result,
        vec!["hkRefPtr<hkpBreakableMultiMaterial::InverseMapping>"]
    );
}

#[test]
fn test_type_404() {
    let result = parse_type("struct hkpBuildJacobianTask::AtomInfo");
    assert_eq!(result, vec!["hkpBuildJacobianTask", "AtomInfo"]);
}

#[test]
fn test_type_405() {
    let result =
        parse_type("struct __cppobj __declspec(align(8)) hkpMoppAabbJob : hkpCollisionQueryJob");
    assert_eq!(result, vec!["hkpMoppAabbJob"]);
}

#[test]
fn test_type_406() {
    let result = parse_type(
        "struct __cppobj hkWorldOperation::ActivateEntity : hkWorldOperation::BaseOperation",
    );
    assert_eq!(result, vec!["hkWorldOperation", "ActivateEntity"]);
}

#[test]
fn test_type_407() {
    let result = parse_type("struct __cppobj hkArrayBase<hkndDecomposeImpl::ConnectionArea>");
    assert_eq!(
        result,
        vec!["hkArrayBase<hkndDecomposeImpl::ConnectionArea>"]
    );
}

#[test]
fn test_type_408() {
    let result = parse_type("struct __declspec(align(8)) Graphics::SBeginCommandBufferParams");
    assert_eq!(result, vec!["Graphics", "SBeginCommandBufferParams"]);
}

#[test]
fn test_type_409() {
    let result = parse_type("struct /*VFT*/ Scaleform::ResouceProvider_vtbl");
    assert_eq!(result, vec!["Scaleform", "ResouceProvider_vtbl"]);
}

#[test]
fn test_type_410() {
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
fn test_type_411() {
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
fn test_type_412() {
    let result = parse_type("struct __cppobj Scaleform::Pickable<Scaleform::GFx::StaticTextDef>");
    assert_eq!(
        result,
        vec!["Scaleform", "Pickable<Scaleform::GFx::StaticTextDef>"]
    );
}

#[test]
fn test_type_413() {
    let result = parse_type("struct jpeg_marker_struct");
    assert_eq!(result, vec!["jpeg_marker_struct"]);
}

#[test]
fn test_type_414() {
    let result = parse_type(
        "struct __cppobj Scaleform::Render::DICommand_CreateTexture : Scaleform::Render::DICommandImpl<Scaleform::Render::DICommand_CreateTexture,Scaleform::Render::DICommand>",
    );
    assert_eq!(
        result,
        vec!["Scaleform", "Render", "DICommand_CreateTexture"]
    );
}

#[test]
fn test_type_415() {
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
fn test_type_416() {
    let result = parse_type("struct /*VFT*/ Scaleform::GFx::AS3::MovieRoot::StickyVarNode_vtbl");
    assert_eq!(
        result,
        vec!["Scaleform", "GFx", "AS3", "MovieRoot", "StickyVarNode_vtbl"]
    );
}

#[test]
fn test_type_417() {
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
fn test_type_418() {
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
fn test_type_419() {
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
fn test_type_420() {
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
fn test_type_421() {
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
fn test_type_422() {
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
fn test_type_423() {
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
fn test_type_424() {
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
fn test_type_425() {
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
fn test_type_426() {
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
fn test_type_427() {
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
fn test_type_428() {
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
fn test_type_429() {
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
fn test_type_430() {
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
fn test_type_431() {
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
fn test_type_432() {
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
fn test_type_433() {
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
fn test_type_434() {
    let result = parse_type(
        "struct __cppobj Scaleform::GFx::AS3::Classes::fl_text::FontStyle : Scaleform::GFx::AS3::Class",
    );
    assert_eq!(
        result,
        vec!["Scaleform", "GFx", "AS3", "Classes", "fl_text", "FontStyle"]
    );
}

#[test]
fn test_type_435() {
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
fn test_type_436() {
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
fn test_type_437() {
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
fn test_type_438() {
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
fn test_type_439() {
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
fn test_type_440() {
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
fn test_type_441() {
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
fn test_type_442() {
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
fn test_type_443() {
    let result = parse_type("struct DIDEVICEINSTANCEW");
    assert_eq!(result, vec!["DIDEVICEINSTANCEW"]);
}

#[test]
fn test_type_444() {
    let result = parse_type("const struct __declspec(align(8)) D2D1_HWND_RENDER_TARGET_PROPERTIES");
    assert_eq!(result, vec!["D2D1_HWND_RENDER_TARGET_PROPERTIES"]);
}

#[test]
fn test_type_445() {
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
fn test_type_446() {
    let result = parse_type("struct __cppobj std::_Pair_base<CRuntimeContainer,CRigidObject *>");
    assert_eq!(
        result,
        vec!["std", "_Pair_base<CRuntimeContainer,CRigidObject *>"]
    );
}

#[test]
fn test_type_447() {
    let result = parse_type(
        "struct __cppobj std::vector<NVehicle_Debug::SDestroyedVehicleLog> : std::_Vector_val<NVehicle_Debug::SDestroyedVehicleLog>",
    );
    assert_eq!(
        result,
        vec!["std", "vector<NVehicle_Debug::SDestroyedVehicleLog>"]
    );
}

#[test]
fn test_type_448() {
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
fn test_type_449() {
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
fn test_type_450() {
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
fn test_type_451() {
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
fn test_type_452() {
    let result = parse_type(
        "struct __cppobj std::tr1::_Callable_obj<<lambda160>,0> : std::tr1::_Callable_base<<lambda160>,0>",
    );
    assert_eq!(result, vec!["std", "tr1", "_Callable_obj<<lambda160>,0>"]);
}

#[test]
fn test_type_453() {
    let result = parse_type(
        "struct /*VFT*/ COperation_Functor<`anonymous namespace'::<lambda40>,CCallContext>_vtbl",
    );
    assert_eq!(
        result,
        vec!["COperation_Functor<`anonymous namespace'::<lambda40>,CCallContext>_vtbl"]
    );
}

#[test]
fn test_type_454() {
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
fn test_type_455() {
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
fn test_type_456() {
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
fn test_type_457() {
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
fn test_type_458() {
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
fn test_type_459() {
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
fn test_type_460() {
    let result = parse_type(
        "typedef hkndSliceFracture_DefaultStruct _A0xb4945271::hkndSliceFracture_DefaultStruct;",
    );
    assert_eq!(
        result,
        vec!["_A0xb4945271", "hkndSliceFracture_DefaultStruct"]
    );
}

#[test]
fn test_type_461() {
    let result = parse_type(
        "typedef hkHandle<unsigned int,0,hkcdConvexCellsCollection::CellIdDiscriminant> CellId;",
    );
    assert_eq!(result, vec!["CellId"]);
}

#[test]
fn test_type_462() {
    let result = parse_type("typedef <lambda6> _A0x8fd0df16::<lambda6>;");
    assert_eq!(result, vec!["_A0x8fd0df16", "<lambda6>"]);
}

#[test]
fn test_type_463() {
    let result = parse_type("typedef tagBIND_OPTS BIND_OPTS;");
    assert_eq!(result, vec!["BIND_OPTS"]);
}

#[test]
fn test_type_464() {
    let result = parse_type("typedef tagWNDCLASSEXW WNDCLASSEXW;");
    assert_eq!(result, vec!["WNDCLASSEXW"]);
}

#[test]
fn test_type_465() {
    let result = parse_type("typedef unsigned int tval_t;");
    assert_eq!(result, vec!["tval_t"]);
}

#[test]
fn test_type_466() {
    let result = parse_type("typedef unsigned int uint32;");
    assert_eq!(result, vec!["uint32"]);
}

#[test]
fn test_type_467() {
    let result = parse_type("typedef unsigned __int8 *PUCHAR;");
    assert_eq!(result, vec!["*PUCHAR"]);
}

#[test]
fn test_type_468() {
    let result = parse_type("typedef <lambda44> _A0x15f5d329::<lambda44>;");
    assert_eq!(result, vec!["_A0x15f5d329", "<lambda44>"]);
}

#[test]
fn test_type_469() {
    let result = parse_type("typedef hkaiNavVolumeInstance hkaiNavVolumeAccessor;");
    assert_eq!(result, vec!["hkaiNavVolumeAccessor"]);
}

#[test]
fn test_type_470() {
    let result = parse_type("typedef _PROCESSOR_RELATIONSHIP PROCESSOR_RELATIONSHIP;");
    assert_eq!(result, vec!["PROCESSOR_RELATIONSHIP"]);
}

#[test]
fn test_type_471() {
    let result = parse_type("typedef __int64 SInt64;");
    assert_eq!(result, vec!["SInt64"]);
}

#[test]
fn test_type_472() {
    let result = parse_type("typedef unsigned __int16 TTerrainPatchDispType;");
    assert_eq!(result, vec!["TTerrainPatchDispType"]);
}

#[test]
fn test_type_473() {
    let result = parse_type("typedef XML_ParserStruct *XML_Parser;");
    assert_eq!(result, vec!["*XML_Parser"]);
}

#[test]
fn test_type_474() {
    let result = parse_type("typedef tagCOMPOSITIONFORM *LPCOMPOSITIONFORM;");
    assert_eq!(result, vec!["*LPCOMPOSITIONFORM"]);
}

#[test]
fn test_type_475() {
    let result = parse_type("typedef hkTrait::TraitBool<0> TypeIsClass;");
    assert_eq!(result, vec!["TypeIsClass"]);
}

#[test]
fn test_type_476() {
    let result = parse_type("typedef _SYSTEM_MANDATORY_LABEL_ACE SYSTEM_MANDATORY_LABEL_ACE;");
    assert_eq!(result, vec!["SYSTEM_MANDATORY_LABEL_ACE"]);
}

#[test]
fn test_type_477() {
    let result = parse_type("typedef _IMAGE_ROM_OPTIONAL_HEADER IMAGE_ROM_OPTIONAL_HEADER;");
    assert_eq!(result, vec!["IMAGE_ROM_OPTIONAL_HEADER"]);
}

#[test]
fn test_type_478() {
    let result = parse_type("typedef _UNIVERSAL_NAME_INFOW *LPUNIVERSAL_NAME_INFOW;");
    assert_eq!(result, vec!["*LPUNIVERSAL_NAME_INFOW"]);
}

#[test]
fn test_type_479() {
    let result = parse_type("typedef arith_entropy_decoder *arith_entropy_ptr;");
    assert_eq!(result, vec!["*arith_entropy_ptr"]);
}

#[test]
fn test_type_480() {
    let result = parse_type("typedef _ENLISTMENT_CRM_INFORMATION *PENLISTMENT_CRM_INFORMATION;");
    assert_eq!(result, vec!["*PENLISTMENT_CRM_INFORMATION"]);
}

#[test]
fn test_type_481() {
    let result = parse_type("typedef _DIOBJECTDATAFORMAT DIOBJECTDATAFORMAT;");
    assert_eq!(result, vec!["DIOBJECTDATAFORMAT"]);
}

#[test]
fn test_type_482() {
    let result = parse_type("typedef NL_DAD_STATE IP_DAD_STATE;");
    assert_eq!(result, vec!["IP_DAD_STATE"]);
}

#[test]
fn test_type_483() {
    let result = parse_type("typedef <lambda8> _A0x743c4eea::<lambda8>;");
    assert_eq!(result, vec!["_A0x743c4eea", "<lambda8>"]);
}

#[test]
fn test_type_484() {
    let result = parse_type("typedef bool (__fastcall *PredicateFunctionType)(hkVariant);");
    assert_eq!(
        result,
        vec!["(__fastcall *PredicateFunctionType)(hkVariant)"]
    );
}

#[test]
fn test_type_485() {
    let result = parse_type("typedef unsigned __int16 **png_uint_16pp;");
    assert_eq!(result, vec!["**png_uint_16pp"]);
}

#[test]
fn test_type_486() {
    let result = parse_type(
        "typedef boost::date_time::years_duration<boost::gregorian::greg_durations_config> years;",
    );
    assert_eq!(result, vec!["years"]);
}

#[test]
fn test_type_487() {
    let result = parse_type("typedef <lambda15> _A0x0a10f1f7::<lambda15>;");
    assert_eq!(result, vec!["_A0x0a10f1f7", "<lambda15>"]);
}

#[test]
fn test_type_488() {
    let result = parse_type("typedef _TAPE_WRITE_MARKS *PTAPE_WRITE_MARKS;");
    assert_eq!(result, vec!["*PTAPE_WRITE_MARKS"]);
}

#[test]
fn test_type_489() {
    let result = parse_type("typedef const char *PCTSTR;");
    assert_eq!(result, vec!["*PCTSTR"]);
}

#[test]
fn test_type_490() {
    let result = parse_type("typedef tagHW_PROFILE_INFOA *LPHW_PROFILE_INFOA;");
    assert_eq!(result, vec!["*LPHW_PROFILE_INFOA"]);
}

#[test]
fn test_type_491() {
    let result = parse_type("typedef _TAPE_WMI_OPERATIONS *PTAPE_WMI_OPERATIONS;");
    assert_eq!(result, vec!["*PTAPE_WMI_OPERATIONS"]);
}

#[test]
fn test_type_492() {
    let result = parse_type("typedef <lambda106> _A0x2c6d2d03::<lambda106>;");
    assert_eq!(result, vec!["_A0x2c6d2d03", "<lambda106>"]);
}

#[test]
fn test_type_493() {
    let result = parse_type("typedef const wchar_t *PCUWSTR;");
    assert_eq!(result, vec!["*PCUWSTR"]);
}

#[test]
fn test_type_494() {
    let result = parse_type("typedef <lambda10> _A0x743c4eea::<lambda10>;");
    assert_eq!(result, vec!["_A0x743c4eea", "<lambda10>"]);
}

#[test]
fn test_type_495() {
    let result = parse_type("typedef <lambda13> _A0x743c4eea::<lambda13>;");
    assert_eq!(result, vec!["_A0x743c4eea", "<lambda13>"]);
}

#[test]
fn test_type_496() {
    let result = parse_type("typedef _ADMINISTRATOR_POWER_POLICY ADMINISTRATOR_POWER_POLICY;");
    assert_eq!(result, vec!["ADMINISTRATOR_POWER_POLICY"]);
}

#[test]
fn test_type_497() {
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
fn test_type_498() {
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
fn test_type_499() {
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
fn test_type_500() {
    let result = parse_type("struct /*VFT*/ CPlantedExplosiveWeapon_vtbl");
    assert_eq!(result, vec!["CPlantedExplosiveWeapon_vtbl"]);
}
