pub mod instance0;
pub mod instance1;
pub mod instance2;
pub mod instance3;
pub mod instance4;
pub mod instance5;
pub mod instance6;
pub mod instance7;
pub mod instance8;
pub mod instance9;

pub struct QueryRoot;

#[juniper::graphql_object]
impl QueryRoot {
    fn event_instances0() -> instance0::EventInstance0Object {
        instance0::event_instances0()
    }
    fn event_instances1() -> instance1::EventInstance1Object {
        instance1::event_instances1()
    }
    fn event_instances2() -> instance2::EventInstance2Object {
        instance2::event_instances2()
    }
    fn event_instances3() -> instance3::EventInstance3Object {
        instance3::event_instances3()
    }
    fn event_instances4() -> instance4::EventInstance4Object {
        instance4::event_instances4()
    }
    fn event_instances5() -> instance5::EventInstance5Object {
        instance5::event_instances5()
    }
    fn event_instances6() -> instance6::EventInstance6Object {
        instance6::event_instances6()
    }
    fn event_instances7() -> instance7::EventInstance7Object {
        instance7::event_instances7()
    }
    fn event_instances8() -> instance8::EventInstance8Object {
        instance8::event_instances8()
    }
    fn event_instances9() -> instance9::EventInstance9Object {
        instance9::event_instances9()
    }
}

pub struct MutationRoot;

#[juniper::graphql_object]
impl MutationRoot {
    async fn create0(
        input: instance0::EventInstanceCreateInput0,
    ) -> instance0::EventInstance0Object {
        instance0::create0(input).await.unwrap()
    }
    async fn create1(
        input: instance1::EventInstanceCreateInput1,
    ) -> instance1::EventInstance1Object {
        instance1::create1(input).await.unwrap()
    }
    async fn create2(
        input: instance2::EventInstanceCreateInput2,
    ) -> instance2::EventInstance2Object {
        instance2::create2(input).await.unwrap()
    }
    async fn create3(
        input: instance3::EventInstanceCreateInput3,
    ) -> instance3::EventInstance3Object {
        instance3::create3(input).await.unwrap()
    }
    async fn create4(
        input: instance4::EventInstanceCreateInput4,
    ) -> instance4::EventInstance4Object {
        instance4::create4(input).await.unwrap()
    }
    async fn create5(
        input: instance5::EventInstanceCreateInput5,
    ) -> instance5::EventInstance5Object {
        instance5::create5(input).await.unwrap()
    }
    async fn create6(
        input: instance6::EventInstanceCreateInput6,
    ) -> instance6::EventInstance6Object {
        instance6::create6(input).await.unwrap()
    }
    async fn create7(
        input: instance7::EventInstanceCreateInput7,
    ) -> instance7::EventInstance7Object {
        instance7::create7(input).await.unwrap()
    }
    async fn create8(
        input: instance8::EventInstanceCreateInput8,
    ) -> instance8::EventInstance8Object {
        instance8::create8(input).await.unwrap()
    }
    async fn create9(
        input: instance9::EventInstanceCreateInput9,
    ) -> instance9::EventInstance9Object {
        instance9::create9(input).await.unwrap()
    }
}
